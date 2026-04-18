//! Basic usage example of legis-entropy library

use legis_entropy::{LegalAnalyzer, DocumentParser, Result};
use tracing_subscriber::FmtSubscriber;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let _subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    // Create parser and analyzer
    let parser = DocumentParser::new();
    let analyzer = LegalAnalyzer::new("dummy_token", 0.1); // Lower threshold

    // Example URLs for Kazakh legal documents
    let urls = vec![
        "https://adilet.zan.kz/rus/docs/K950001000_",
        "https://adilet.zan.kz/rus/docs/K930001000_",
    ];

    println!("📥 Parsing {} documents...", urls.len());
    let mut documents = Vec::new();

    for url in &urls {
        match parser.parse(url).await {
            Ok(doc) => {
                println!("✓ Parsed: {} ({} chars)", doc.id, doc.text.len());
                documents.push(doc);
            }
            Err(e) => {
                println!("✗ Failed to parse {}: {}", url, e);
            }
        }
    }

    if documents.is_empty() {
        println!("No documents to analyze");
        return Ok(());
    }

    println!("🔍 Analyzing documents...");
    let results = analyzer.analyze(documents).await?;

    // Display results
    println!("\n📊 Analysis Results:");
    println!("Total documents: {}", results.total_documents);
    println!("Pairs analyzed: {}", results.total_pairs_analyzed);
    println!("Similar pairs: {}", results.similar_pairs.len());
    println!("Contradictions: {}", results.contradictions.len());

    if !results.similar_pairs.is_empty() {
        println!("\n🔗 Similar Pairs:");
        for pair in &results.similar_pairs {
            println!("  • {} ↔ {}", pair.doc1_id, pair.doc2_id);
            println!("    Similarity: {:.2}%", pair.similarity_score * 100.0);
            if pair.is_contradiction {
                println!("    ⚠️  CONTRADICTION DETECTED!");
            }
        }
    }

    // Save results to JSON
    let json = serde_json::to_string_pretty(&results)?;
    std::fs::write("analysis_results.json", &json)?;
    println!("\n💾 Results saved to analysis_results.json");

    Ok(())
}
