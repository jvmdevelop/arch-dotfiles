use crate::data::parser::parse_raw_document;
use crate::embedding::{Embedder, cosine_sim};
use crate::graph::DocumentGraph;
use crate::analyzer::DocumentAnalyzer;
use crate::issue::IssueDetector;
use crate::reporter::{ReportGenerator, ReportConfig, ReportFormat};

mod data;
mod embedding;
mod graph;
mod analyzer;
mod issue;
mod reporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Legal Document Analysis Pipeline Starting...");
    
    // Example URLs for analysis
    let urls = vec![
        "https://adilet.zan.kz/rus/docs/K950001000_",
        "https://adilet.zan.kz/rus/docs/K990000123_",
        "https://adilet.zan.kz/rus/docs/K040000456_",
    ];

    println!("📥 Parsing {} documents...", urls.len());
    let mut documents = Vec::new();
    
    for url in &urls {
        match parse_raw_document(url).await {
            Ok(doc) => {
                println!("✅ Successfully parsed: {}", doc.title);
                documents.push(doc);
            }
            Err(e) => {
                eprintln!("❌ Failed to parse {}: {}", url, e);
            }
        }
    }

    if documents.is_empty() {
        println!("⚠️  No documents were successfully parsed. Exiting.");
        return Ok(());
    }

    println!("📊 Building document relationship graph...");
    let graph = DocumentGraph::build_from_documents(&documents);
    let graph_stats = graph.get_document_statistics();
    println!("✅ Graph built: {} nodes, {} edges", graph_stats.total_documents, graph_stats.total_relationships);

    println!("🧠 Initializing analyzer with embeddings...");
    let analyzer = DocumentAnalyzer::new().with_embeddings()?;
    println!("✅ Analyzer ready");

    println!("🔬 Analyzing documents for issues...");
    let analysis_result = analyzer.analyze_documents(&documents, &graph);
    println!("✅ Analysis complete:");
    println!("   - Conflicts: {}", analysis_result.conflicts.len());
    println!("   - Duplicates: {}", analysis_result.duplicates.len());
    println!("   - Outdated: {}", analysis_result.outdated.len());
    println!("   - Inconsistencies: {}", analysis_result.inconsistencies.len());
    println!("   - Semantic similarities: {}", analysis_result.semantic_similarities.len());

    println!("🚨 Detecting and prioritizing issues...");
    let issue_detector = IssueDetector::new().with_confidence_threshold(0.7);
    let issue_report = issue_detector.detect_issues(
        &analysis_result.conflicts,
        &analysis_result.duplicates,
        &analysis_result.outdated,
        &analysis_result.inconsistencies,
        &analysis_result.semantic_similarities,
    );
    println!("✅ Issue detection complete: {} total issues", issue_report.issues.len());

    println!("📋 Generating comprehensive report...");
    let report_config = ReportConfig {
        include_graph: true,
        include_statistics: true,
        include_recommendations: true,
        output_format: ReportFormat::Markdown,
        detail_level: crate::reporter::DetailLevel::Standard,
    };

    let report_generator = ReportGenerator::new(report_config);
    let comprehensive_report = report_generator.generate_comprehensive_report(
        issue_report,
        &analysis_result,
        &graph,
        graph_stats,
    );

    // Generate reports in different formats
    println!("💾 Saving reports...");
    
    // Markdown report
    let markdown_report = report_generator.export_to_markdown(&comprehensive_report);
    std::fs::write("legal_analysis_report.md", markdown_report)?;
    println!("✅ Markdown report saved: legal_analysis_report.md");

    // JSON report
    let json_report = report_generator.export_to_json(&comprehensive_report)?;
    std::fs::write("legal_analysis_report.json", json_report)?;
    println!("✅ JSON report saved: legal_analysis_report.json");

    // HTML report
    let html_report = report_generator.export_to_html(&comprehensive_report);
    std::fs::write("legal_analysis_report.html", html_report)?;
    println!("✅ HTML report saved: legal_analysis_report.html");

    // Graph visualization
    let dot_content = graph.export_to_dot();
    std::fs::write("document_graph.dot", dot_content)?;
    println!("✅ Graph visualization saved: document_graph.dot");

    // Print summary
    println!("\n📊 Analysis Summary:");
    println!("═══════════════════════════════════════");
    println!("Documents Analyzed: {}", comprehensive_report.metadata.documents_analyzed);
    println!("Total Issues: {}", comprehensive_report.issue_report.summary.total_issues);
    println!("Critical Issues: {}", comprehensive_report.issue_report.summary.critical_issues);
    println!("High Priority Issues: {}", comprehensive_report.issue_report.summary.high_issues);
    println!("Auto-fixable Issues: {}", comprehensive_report.issue_report.summary.auto_fixable_count);
    println!("Overall Health Score: {:.1}%", comprehensive_report.executive_summary.overall_health_score * 100.0);
    println!("Analysis Duration: {}ms", comprehensive_report.metadata.analysis_duration_ms);
    println!("═══════════════════════════════════════");

    if !comprehensive_report.executive_summary.immediate_actions.is_empty() {
        println!("\n🚨 Immediate Actions Required:");
        for action in &comprehensive_report.executive_summary.immediate_actions {
            println!("  • {}", action);
        }
    }

    if !comprehensive_report.recommendations.is_empty() {
        println!("\n💡 Top Recommendations:");
        for (i, rec) in comprehensive_report.recommendations.iter().take(3).enumerate() {
            println!("  {}. {} ({:?} priority)", i + 1, rec.title, rec.priority);
        }
    }

    println!("\n🎉 Analysis complete! Check the generated reports for detailed information.");

    Ok(())
}
