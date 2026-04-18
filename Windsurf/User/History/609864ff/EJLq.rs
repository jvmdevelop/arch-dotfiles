mod analyzer;
mod errors;
mod hf;
mod models;
mod parser;

use analyzer::LegalAnalyzer;
use errors::{AppError, Result};
use parser::DocumentParser;
use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

const DEMO_URLS: &[&str] = &[
    "https://adilet.zan.kz/rus/docs/K950001000_",
    "https://adilet.zan.kz/rus/docs/K930001000_",
    "https://adilet.zan.kz/rus/docs/K2600000000",
    "https://adilet.zan.kz/rus/docs/Z1100000403",
    "https://adilet.zan.kz/rus/docs/Z0700000254_",
];

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let _subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).init();

    let hf_token = env::var("HF_TOKEN")
        .map_err(|_| AppError::Config("HF_TOKEN environment variable not set".to_string()))?;

    let parser = DocumentParser::new();
    let analyzer = LegalAnalyzer::new(&hf_token, 0.5);

    info!("📥 Parsing {} documents...", DEMO_URLS.len());
    let mut documents = Vec::new();

    for url in DEMO_URLS {
        match parser.parse(url).await {
            Ok(doc) => {
                info!("parsed: {} ({} chars)", doc.id, doc.text.len());
                documents.push(doc);
            }
            Err(e) => {
                info!("failed to parse {}: {}", url, e);
            }
        }
    }

    if documents.is_empty() {
        info!("no documents to analyze");
        return Ok(());
    }

    info!("🔍 Analyzing documents...");
    let results = analyzer.analyze(documents).await?;

    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write("results.json", &json)?;

    Ok(())
}
