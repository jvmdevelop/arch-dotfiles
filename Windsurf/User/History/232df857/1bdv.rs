use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::issue::{IssueReport, Issue, Severity, Priority};
use crate::graph::{DocumentGraph, GraphStatistics};
use crate::analyzer::AnalysisResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    pub include_graph: bool,
    pub include_statistics: bool,
    pub include_recommendations: bool,
    pub output_format: ReportFormat,
    pub detail_level: DetailLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    Markdown,
    Json,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetailLevel {
    Summary,
    Standard,
    Detailed,
    Comprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveReport {
    pub metadata: ReportMetadata,
    pub executive_summary: ExecutiveSummary,
    pub issue_report: IssueReport,
    pub graph_statistics: Option<GraphStatistics>,
    pub graph_visualization: Option<String>,
    pub recommendations: Vec<Recommendation>,
    pub appendix: Appendix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub generated_at: String,
    pub report_version: String,
    pub analyzer_version: String,
    pub documents_analyzed: usize,
    pub analysis_duration_ms: u64,
    pub confidence_threshold: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    pub total_issues: usize,
    pub critical_issues: usize,
    pub high_priority_issues: usize,
    pub auto_fixable_issues: usize,
    pub overall_health_score: f32,
    pub key_findings: Vec<String>,
    pub immediate_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub affected_issues: Vec<String>,
    pub estimated_effort: EffortEstimate,
    pub impact_assessment: ImpactAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    ConflictResolution,
    DuplicateManagement,
    DocumentMaintenance,
    ProcessImprovement,
    TechnicalDebt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffortEstimate {
    pub hours_min: u32,
    pub hours_max: u32,
    pub complexity: Complexity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Complexity {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub risk_reduction: f32,
    pub compliance_improvement: f32,
    pub efficiency_gain: f32,
    pub stakeholder_impact: StakeholderImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StakeholderImpact {
    Minimal,
    Moderate,
    Significant,
    Major,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appendix {
    pub technical_details: TechnicalDetails,
    pub methodology: Methodology,
    pub limitations: Vec<String>,
    pub glossary: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalDetails {
    pub analysis_algorithms: Vec<String>,
    pub confidence_calculation: String,
    pub severity_scoring: HashMap<String, String>,
    pub data_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Methodology {
    pub analysis_approach: String,
    pub conflict_detection_rules: Vec<String>,
    pub similarity_thresholds: HashMap<String, f32>,
    pub quality_metrics: Vec<String>,
}

pub struct ReportGenerator {
    config: ReportConfig,
}

impl ReportGenerator {
    pub fn new(config: ReportConfig) -> Self {
        Self { config }
    }

    pub fn generate_comprehensive_report(
        &self,
        issue_report: IssueReport,
        analysis_result: &AnalysisResult,
        graph: &DocumentGraph,
        graph_statistics: GraphStatistics,
    ) -> ComprehensiveReport {
        let metadata = self.create_metadata(&issue_report);
        let executive_summary = self.create_executive_summary(&issue_report);
        let recommendations = self.generate_recommendations(&issue_report, analysis_result);
        let appendix = self.create_appendix();
        
        let graph_visualization = if self.config.include_graph {
            Some(graph.export_to_dot())
        } else {
            None
        };

        let graph_statistics = if self.config.include_statistics {
            Some(graph_statistics)
        } else {
            None
        };

        ComprehensiveReport {
            metadata,
            executive_summary,
            issue_report,
            graph_statistics,
            graph_visualization,
            recommendations,
            appendix,
        }
    }

    pub fn export_to_markdown(&self, report: &ComprehensiveReport) -> String {
        let mut markdown = String::new();
        
        markdown.push_str("# Legal Document Analysis Report\n\n");
        markdown.push_str(&self.format_metadata_markdown(&report.metadata));
        
        markdown.push_str("## Executive Summary\n\n");
        markdown.push_str(&self.format_executive_summary_markdown(&report.executive_summary));
        
        markdown.push_str("## Issues Found\n\n");
        markdown.push_str(&self.format_issues_markdown(&report.issue_report.issues));
        
        if let Some(stats) = &report.graph_statistics {
            markdown.push_str("## Document Relationship Graph\n\n");
            markdown.push_str(&self.format_graph_statistics_markdown(stats));
        }
        
        if let Some(dot_content) = &report.graph_visualization {
            markdown.push_str("## Graph Visualization\n\n");
            markdown.push_str("```dot\n");
            markdown.push_str(dot_content);
            markdown.push_str("\n```\n\n");
        }
        
        if self.config.include_recommendations {
            markdown.push_str("## Recommendations\n\n");
            markdown.push_str(&self.format_recommendations_markdown(&report.recommendations));
        }
        
        markdown.push_str("## Appendix\n\n");
        markdown.push_str(&self.format_appendix_markdown(&report.appendix));
        
        markdown
    }

    pub fn export_to_json(&self, report: &ComprehensiveReport) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(report)
    }

    pub fn export_to_html(&self, report: &ComprehensiveReport) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<title>Legal Document Analysis Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 40px; }\n");
        html.push_str(".severity-critical { color: #d32f2f; font-weight: bold; }\n");
        html.push_str(".severity-high { color: #f57c00; font-weight: bold; }\n");
        html.push_str(".severity-medium { color: #fbc02d; }\n");
        html.push_str(".severity-low { color: #388e3c; }\n");
        html.push_str(".severity-info { color: #1976d2; }\n");
        html.push_str("table { border-collapse: collapse; width: 100%; margin: 20px 0; }\n");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("th { background-color: #f2f2f2; }\n");
        html.push_str(".summary-card { background: #f9f9f9; padding: 20px; margin: 20px 0; border-radius: 8px; }\n");
        html.push_str("</style>\n</head>\n<body>\n");
        
        html.push_str("<h1>Legal Document Analysis Report</h1>\n");
        html.push_str(&self.format_metadata_html(&report.metadata));
        html.push_str(&self.format_executive_summary_html(&report.executive_summary));
        html.push_str(&self.format_issues_html(&report.issue_report.issues));
        
        if let Some(stats) = &report.graph_statistics {
            html.push_str(&self.format_graph_statistics_html(stats));
        }
        
        html.push_str(&self.format_recommendations_html(&report.recommendations));
        html.push_str("</body>\n</html>");
        
        html
    }

    fn create_metadata(&self, issue_report: &IssueReport) -> ReportMetadata {
        ReportMetadata {
            generated_at: issue_report.metadata.generated_at.clone(),
            report_version: "1.0.0".to_string(),
            analyzer_version: issue_report.metadata.analyzer_version.clone(),
            documents_analyzed: issue_report.metadata.documents_analyzed,
            analysis_duration_ms: issue_report.metadata.analysis_duration_ms,
            confidence_threshold: issue_report.metadata.confidence_threshold,
        }
    }

    fn create_executive_summary(&self, issue_report: &IssueReport) -> ExecutiveSummary {
        let overall_health_score = self.calculate_health_score(&issue_report.summary);
        let key_findings = self.extract_key_findings(&issue_report.summary);
        let immediate_actions = self.extract_immediate_actions(&issue_report.issues);

        ExecutiveSummary {
            total_issues: issue_report.summary.total_issues,
            critical_issues: issue_report.summary.critical_issues,
            high_priority_issues: issue_report.summary.high_issues,
            auto_fixable_issues: issue_report.summary.auto_fixable_count,
            overall_health_score,
            key_findings,
            immediate_actions,
        }
    }

    fn generate_recommendations(&self, issue_report: &IssueReport, analysis_result: &AnalysisResult) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        if !analysis_result.conflicts.is_empty() {
            recommendations.push(Recommendation {
                id: "REC_001".to_string(),
                category: RecommendationCategory::ConflictResolution,
                priority: Priority::Immediate,
                title: "Resolve Document Conflicts".to_string(),
                description: format!("Address {} conflicts found between legal documents", analysis_result.conflicts.len()),
                affected_issues: issue_report.issues
                    .iter()
                    .filter(|i| matches!(i.issue_type, crate::issue::IssueType::Conflict))
                    .map(|i| i.id.clone())
                    .collect(),
                estimated_effort: EffortEstimate {
                    hours_min: 8,
                    hours_max: 40,
                    complexity: Complexity::High,
                },
                impact_assessment: ImpactAssessment {
                    risk_reduction: 0.8,
                    compliance_improvement: 0.9,
                    efficiency_gain: 0.6,
                    stakeholder_impact: StakeholderImpact::Significant,
                },
            });
        }

        if !analysis_result.duplicates.is_empty() {
            recommendations.push(Recommendation {
                id: "REC_002".to_string(),
                category: RecommendationCategory::DuplicateManagement,
                priority: Priority::High,
                title: "Consolidate Duplicate Documents".to_string(),
                description: format!("Review and consolidate {} duplicate or near-duplicate documents", analysis_result.duplicates.len()),
                affected_issues: issue_report.issues
                    .iter()
                    .filter(|i| matches!(i.issue_type, crate::issue::IssueType::Duplicate))
                    .map(|i| i.id.clone())
                    .collect(),
                estimated_effort: EffortEstimate {
                    hours_min: 4,
                    hours_max: 16,
                    complexity: Complexity::Medium,
                },
                impact_assessment: ImpactAssessment {
                    risk_reduction: 0.4,
                    compliance_improvement: 0.3,
                    efficiency_gain: 0.7,
                    stakeholder_impact: StakeholderImpact::Moderate,
                },
            });
        }

        if !analysis_result.outdated.is_empty() {
            recommendations.push(Recommendation {
                id: "REC_003".to_string(),
                category: RecommendationCategory::DocumentMaintenance,
                priority: Priority::Medium,
                title: "Archive Outdated Documents".to_string(),
                description: format!("Process {} outdated documents according to retention policies", analysis_result.outdated.len()),
                affected_issues: issue_report.issues
                    .iter()
                    .filter(|i| matches!(i.issue_type, crate::issue::IssueType::Outdated))
                    .map(|i| i.id.clone())
                    .collect(),
                estimated_effort: EffortEstimate {
                    hours_min: 2,
                    hours_max: 8,
                    complexity: Complexity::Low,
                },
                impact_assessment: ImpactAssessment {
                    risk_reduction: 0.3,
                    compliance_improvement: 0.5,
                    efficiency_gain: 0.4,
                    stakeholder_impact: StakeholderImpact::Minimal,
                },
            });
        }

        recommendations
    }

    fn create_appendix(&self) -> Appendix {
        let mut glossary = HashMap::new();
        glossary.insert("Conflict".to_string(), "A situation where two or more documents contain contradictory information".to_string());
        glossary.insert("Duplicate".to_string(), "Documents that are identical or substantially similar in content".to_string());
        glossary.insert("Outdated".to_string(), "Documents that are no longer current or have been superseded".to_string());
        glossary.insert("Semantic Similarity".to_string(), "Documents that have similar meaning but different wording".to_string());

        Appendix {
            technical_details: TechnicalDetails {
                analysis_algorithms: vec![
                    "Text similarity using cosine similarity".to_string(),
                    "Regular expression pattern matching".to_string(),
                    "Graph-based relationship analysis".to_string(),
                    "Embedding-based semantic analysis".to_string(),
                ],
                confidence_calculation: "Based on evidence strength and pattern matching confidence".to_string(),
                severity_scoring: HashMap::from([
                    ("Conflict".to_string(), "Critical - Immediate attention required".to_string()),
                    ("Duplicate".to_string(), "High - Review and consolidation needed".to_string()),
                    ("Outdated".to_string(), "Medium - Maintenance task".to_string()),
                ]),
                data_sources: vec![
                    "Document text content".to_string(),
                    "Document metadata".to_string(),
                    "Cross-reference links".to_string(),
                    "Document status information".to_string(),
                ],
            },
            methodology: Methodology {
                analysis_approach: "Multi-faceted analysis combining textual, structural, and semantic analysis".to_string(),
                conflict_detection_rules: vec![
                    "Repeal status conflicts".to_string(),
                    "Amendment timeline validation".to_string(),
                    "Contradiction pattern matching".to_string(),
                ],
                similarity_thresholds: HashMap::from([
                    ("text_similarity".to_string(), 0.8),
                    ("semantic_similarity".to_string(), 0.7),
                ]),
                quality_metrics: vec![
                    "Confidence score".to_string(),
                    "Evidence strength".to_string(),
                    "Explainability score".to_string(),
                ],
            },
            limitations: vec![
                "Analysis limited to available document content".to_string(),
                "Context-dependent conflicts may require manual review".to_string(),
                "Semantic analysis depends on model quality".to_string(),
                "Cross-reference validation requires complete document registry".to_string(),
            ],
            glossary,
        }
    }

    fn calculate_health_score(&self, summary: &crate::issue::IssueSummary) -> f32 {
        let total_weight = summary.critical_issues as f32 * 10.0 +
                          summary.high_issues as f32 * 5.0 +
                          summary.medium_issues as f32 * 2.0 +
                          summary.low_issues as f32 * 1.0 +
                          summary.info_issues as f32 * 0.5;
        
        let max_possible_score = summary.total_issues as f32 * 10.0;
        
        if max_possible_score > 0.0 {
            1.0 - (total_weight / max_possible_score).min(1.0)
        } else {
            1.0
        }
    }

    fn extract_key_findings(&self, summary: &crate::issue::IssueSummary) -> Vec<String> {
        let mut findings = Vec::new();
        
        if summary.critical_issues > 0 {
            findings.push(format!("{} critical issues requiring immediate attention", summary.critical_issues));
        }
        
        if summary.high_issues > 0 {
            findings.push(format!("{} high-priority issues identified", summary.high_issues));
        }
        
        if summary.auto_fixable_count > 0 {
            findings.push(format!("{} issues can be automatically resolved", summary.auto_fixable_count));
        }
        
        findings
    }

    fn extract_immediate_actions(&self, issues: &[Issue]) -> Vec<String> {
        issues
            .iter()
            .filter(|i| i.priority == Priority::Immediate)
            .take(5)
            .map(|i| format!("{}: {}", i.id, i.title))
            .collect()
    }

    fn format_metadata_markdown(&self, metadata: &ReportMetadata) -> String {
        format!(
            "**Generated:** {}\n**Documents Analyzed:** {}\n**Analysis Duration:** {}ms\n**Confidence Threshold:** {:.2}\n\n",
            metadata.generated_at,
            metadata.documents_analyzed,
            metadata.analysis_duration_ms,
            metadata.confidence_threshold
        )
    }

    fn format_executive_summary_markdown(&self, summary: &ExecutiveSummary) -> String {
        let mut result = String::new();
        result.push_str(&format!("**Overall Health Score:** {:.1}%\n\n", summary.overall_health_score * 100.0));
        result.push_str(&format!("- **Total Issues:** {}\n", summary.total_issues));
        result.push_str(&format!("- **Critical Issues:** {}\n", summary.critical_issues));
        result.push_str(&format!("- **High Priority:** {}\n", summary.high_priority_issues));
        result.push_str(&format!("- **Auto-fixable:** {}\n\n", summary.auto_fixable_issues));
        
        if !summary.key_findings.is_empty() {
            result.push_str("### Key Findings\n\n");
            for finding in &summary.key_findings {
                result.push_str(&format!("- {}\n", finding));
            }
            result.push_str("\n");
        }
        
        if !summary.immediate_actions.is_empty() {
            result.push_str("### Immediate Actions Required\n\n");
            for action in &summary.immediate_actions {
                result.push_str(&format!("- {}\n", action));
            }
            result.push_str("\n");
        }
        
        result
    }

    fn format_issues_markdown(&self, issues: &[Issue]) -> String {
        let mut result = String::new();
        
        let mut grouped = HashMap::new();
        for issue in issues {
            grouped.entry(&issue.severity).or_insert_with(Vec::new).push(issue);
        }
        
        for severity in [Severity::Critical, Severity::High, Severity::Medium, Severity::Low, Severity::Info] {
            if let Some(severity_issues) = grouped.get(&severity) {
                result.push_str(&format!("### {} Issues ({})\n\n", format!("{:?}", severity), severity_issues.len()));
                
                for issue in severity_issues {
                    result.push_str(&format!("#### {} - {}\n\n", issue.id, issue.title));
                    result.push_str(&format!("**Description:** {}\n\n", issue.description));
                    result.push_str(&format!("**Affected Documents:** {}\n\n", issue.affected_documents.join(", ")));
                    result.push_str(&format!("**Confidence:** {:.2}\n\n", issue.confidence));
                    
                    if !issue.evidence.is_empty() {
                        result.push_str("**Evidence:**\n");
                        for evidence in &issue.evidence {
                            result.push_str(&format!- {}\n", evidence));
                        }
                        result.push_str("\n");
                    }
                    
                    if !issue.recommendations.is_empty() {
                        result.push_str("**Recommendations:**\n");
                        for rec in &issue.recommendations {
                            result.push_str(&format!- {}\n", rec));
                        }
                        result.push_str("\n");
                    }
                    
                    result.push_str("---\n\n");
                }
            }
        }
        
        result
    }

    fn format_graph_statistics_markdown(&self, stats: &GraphStatistics) -> String {
        let mut result = String::new();
        result.push_str(&format!("- **Total Documents:** {}\n", stats.total_documents));
        result.push_str(&format!("- **Total Relationships:** {}\n", stats.total_relationships));
        result.push_str(&format!("- **Average Link Strength:** {:.2}\n", stats.average_link_strength));
        result.push_str(&format!("- **Cycles Found:** {}\n\n", stats.cycles_found));
        
        if !stats.link_type_distribution.is_empty() {
            result.push_str("**Link Type Distribution:**\n");
            for (link_type, count) in &stats.link_type_distribution {
                result.push_str(&format!- {}: {}\n", link_type, count));
            }
            result.push_str("\n");
        }
        
        result
    }

    fn format_recommendations_markdown(&self, recommendations: &[Recommendation]) -> String {
        let mut result = String::new();
        
        for rec in recommendations {
            result.push_str(&format!("### {} - {}\n\n", rec.id, rec.title));
            result.push_str(&format!("**Priority:** {:?}\n\n", rec.priority));
            result.push_str(&format!("**Description:** {}\n\n", rec.description));
            result.push_str(&format!("**Estimated Effort:** {}-{} hours ({:?})\n\n", 
                                   rec.estimated_effort.hours_min, 
                                   rec.estimated_effort.hours_max,
                                   rec.estimated_effort.complexity));
            
            if !rec.affected_issues.is_empty() {
                result.push_str(&format!("**Related Issues:** {}\n\n", rec.affected_issues.join(", ")));
            }
            
            result.push_str("---\n\n");
        }
        
        result
    }

    fn format_appendix_markdown(&self, appendix: &Appendix) -> String {
        let mut result = String::new();
        
        result.push_str("### Technical Details\n\n");
        result.push_str("**Analysis Algorithms:**\n");
        for algo in &appendix.technical_details.analysis_algorithms {
            result.push_str(&format!- {}\n", algo));
        }
        result.push_str("\n");
        
        result.push_str("### Methodology\n\n");
        result.push_str(&format!("**Analysis Approach:** {}\n\n", appendix.methodology.analysis_approach));
        
        result.push_str("### Limitations\n\n");
        for limitation in &appendix.limitations {
            result.push_str(&format!- {}\n, limitation));
        }
        result.push_str("\n");
        
        result.push_str("### Glossary\n\n");
        for (term, definition) in &appendix.glossary {
            result.push_str(&format!("**{}:** {}\n\n", term, definition));
        }
        
        result
    }

    fn format_metadata_html(&self, metadata: &ReportMetadata) -> String {
        format!(
            "<div class='summary-card'>
                <h2>Report Metadata</h2>
                <p><strong>Generated:</strong> {}</p>
                <p><strong>Documents Analyzed:</strong> {}</p>
                <p><strong>Analysis Duration:</strong> {}ms</p>
                <p><strong>Confidence Threshold:</strong> {:.2}</p>
            </div>",
            metadata.generated_at,
            metadata.documents_analyzed,
            metadata.analysis_duration_ms,
            metadata.confidence_threshold
        )
    }

    fn format_executive_summary_html(&self, summary: &ExecutiveSummary) -> String {
        let mut result = String::new();
        result.push_str("<div class='summary-card'>\n");
        result.push_str("<h2>Executive Summary</h2>\n");
        result.push_str(&format!("<p><strong>Overall Health Score:</strong> {:.1}%</p>\n", summary.overall_health_score * 100.0));
        
        result.push_str("<table>\n");
        result.push_str("<tr><th>Metric</th><th>Count</th></tr>\n");
        result.push_str(&format!("<tr><td>Total Issues</td><td>{}</td></tr>\n", summary.total_issues));
        result.push_str(&format!("<tr><td>Critical Issues</td><td>{}</td></tr>\n", summary.critical_issues));
        result.push_str(&format!("<tr><td>High Priority</td><td>{}</td></tr>\n", summary.high_priority_issues));
        result.push_str(&format!("<tr><td>Auto-fixable</td><td>{}</td></tr>\n", summary.auto_fixable_issues));
        result.push_str("</table>\n");
        result.push_str("</div>\n");
        
        result
    }

    fn format_issues_html(&self, issues: &[Issue]) -> String {
        let mut result = String::new();
        result.push_str("<h2>Issues Found</h2>\n");
        result.push_str("<table>\n");
        result.push_str("<tr><th>ID</th><th>Title</th><th>Severity</th><th>Priority</th><th>Confidence</th><th>Auto-fixable</th></tr>\n");
        
        for issue in issues {
            let severity_class = format!("severity-{:?}", issue.severity).to_lowercase();
            result.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td class='{}'>{:?}</td><td>{:?}</td><td>{:.2}</td><td>{}</td></tr>\n",
                issue.id,
                issue.title,
                severity_class,
                issue.severity,
                issue.priority,
                issue.confidence,
                if issue.auto_fixable { "Yes" } else { "No" }
            ));
        }
        
        result.push_str("</table>\n");
        result
    }

    fn format_graph_statistics_html(&self, stats: &GraphStatistics) -> String {
        let mut result = String::new();
        result.push_str("<h2>Graph Statistics</h2>\n");
        result.push_str("<table>\n");
        result.push_str("<tr><th>Metric</th><th>Value</th></tr>\n");
        result.push_str(&format!("<tr><td>Total Documents</td><td>{}</td></tr>\n", stats.total_documents));
        result.push_str(&format!("<tr><td>Total Relationships</td><td>{}</td></tr>\n", stats.total_relationships));
        result.push_str(&format!("<tr><td>Average Link Strength</td><td>{:.2}</td></tr>\n", stats.average_link_strength));
        result.push_str(&format!("<tr><td>Cycles Found</td><td>{}</td></tr>\n", stats.cycles_found));
        result.push_str("</table>\n");
        result
    }

    fn format_recommendations_html(&self, recommendations: &[Recommendation]) -> String {
        let mut result = String::new();
        result.push_str("<h2>Recommendations</h2>\n");
        
        for rec in recommendations {
            result.push_str("<div class='summary-card'>\n");
            result.push_str(&format!("<h3>{} - {}</h3>\n", rec.id, rec.title));
            result.push_str(&format!("<p><strong>Priority:</strong> {:?}</p>\n", rec.priority));
            result.push_str(&format!("<p><strong>Description:</strong> {}</p>\n", rec.description));
            result.push_str(&format!("<p><strong>Estimated Effort:</strong> {}-{} hours ({:?})</p>\n",
                                   rec.estimated_effort.hours_min,
                                   rec.estimated_effort.hours_max,
                                   rec.estimated_effort.complexity));
            result.push_str("</div>\n");
        }
        
        result
    }
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            include_graph: true,
            include_statistics: true,
            include_recommendations: true,
            output_format: ReportFormat::Markdown,
            detail_level: DetailLevel::Standard,
        }
    }
}
