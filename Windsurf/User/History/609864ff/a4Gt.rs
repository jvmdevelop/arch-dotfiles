mod analyzer;
mod errors;
mod hf;
mod models;
mod parser;

use analyzer::LegalAnalyzer;
use errors::Result;
use parser::DocumentParser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use std::env;

const HF_TOKEN: &str = env!("HF_TOKEN");

const DEMO_URLS: &[&str] = &[
    "https://adilet.zan.kz/rus/docs/K950001000_",
    "https://adilet.zan.kz/rus/docs/K930001000_",
    "https://adilet.zan.kz/rus/docs/K2600000000",
    "https://adilet.zan.kz/rus/docs/Z1100000403",
    "https://adilet.zan.kz/rus/docs/Z0700000254_",
];

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).init();

    let parser = DocumentParser::new();
    let analyzer = LegalAnalyzer::new(HF_TOKEN, 0.5);

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

    println!("Всего документов: {}", results.total_documents);
    println!("Проанализировано пар: {}", results.total_pairs_analyzed);
    println!("Похожих пар: {}", results.similar_pairs.len());
    println!("Противоречий: {}", results.contradictions.len());

    if !results.contradictions.is_empty() {
        println!("\n🚩 НАЙДЕНЫ ПРОТИВОРЕЧИЯ:\n");
        for pair in &results.contradictions {
            println!("  • {} ↔ {}", pair.doc1_id, pair.doc2_id);
            println!("    Сходство: {:.2}%", pair.similarity_score * 100.0);
            println!("    Статус 1: {:?}", pair.doc1_status);
            println!("    Статус 2: {:?}", pair.doc2_status);
            println!();
        }
    }

    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write("results.json", &json)?;
    info!("💾 Results saved to results.json");

    Ok(())
}
