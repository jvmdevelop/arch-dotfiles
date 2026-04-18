use le_common::{init_metrics, init_tracing};
use le_proto::le::v1::{
    similarity_service_server::{SimilarityService, SimilarityServiceServer},
    SimilarityHit, SimilarityRequest, SimilarityResponse,
};
use le_storage::{connect, repo::migrate, Db};
use sqlx::Row;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Clone)]
struct State {
    db: Db,
}

struct SimilaritySvc {
    state: State,
}

impl SimilaritySvc {
    fn new(state: State) -> Self {
        Self { state }
    }
}

fn env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

#[tonic::async_trait]
impl SimilarityService for SimilaritySvc {
    async fn search(
        &self,
        request: Request<SimilarityRequest>,
    ) -> Result<Response<SimilarityResponse>, Status> {
        let req = request.into_inner();
        let qid = req.query.as_ref().and_then(|d| Uuid::parse_str(&d.id).ok());
        let top_k = req.top_k.max(1).min(100) as i64;

        let hits = if let Some(norm_id) = qid {
            let rows = sqlx::query::<sqlx::Postgres>(
                r#"
                SELECT e2.norm_id AS norm_id,
                       1 - (e1.embedding <=> e2.embedding) AS score
                FROM embeddings e1
                JOIN embeddings e2 ON e2.model_id = e1.model_id
                WHERE e1.norm_id = $1 AND e2.norm_id <> $1
                ORDER BY e1.embedding <=> e2.embedding ASC
                LIMIT $2
                "#,
            )
            .bind(norm_id)
            .bind(top_k)
            .fetch_all(&self.state.db)
            .await
            .map_err(|e| Status::internal(format!("db similarity: {e}")))?;

            rows.into_iter()
                .filter_map(|r| {
                    let nid: Uuid = r.try_get("norm_id").ok()?;
                    let score: f32 = r.try_get("score").ok()?;
                    Some(SimilarityHit {
                        doc_id: nid.to_string(),
                        score,
                    })
                })
                .collect()
        } else {
            vec![]
        };

        Ok(Response::new(SimilarityResponse { hits }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let _metrics = init_metrics()?;

    let database_url = env("DATABASE_URL", "postgres://postgres:postgres@localhost:5432/legis_entropy");
    let db = connect(&database_url).await?;
    if let Err(e) = migrate(&db).await {
        warn!(error=%e, "migrations failed");
    }

    let host = env("LE__SERVICE__HOST", "0.0.0.0");
    let port = env("LE__SERVICE__PORT", "50065");
    let addr = format!("{host}:{port}").parse()?;

    info!(%addr, "starting similarity-service");

    Server::builder()
        .add_service(SimilarityServiceServer::new(SimilaritySvc::new(State { db })))
        .serve(addr)
        .await?;

    Ok(())
}
