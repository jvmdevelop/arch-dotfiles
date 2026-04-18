use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::analyzer::{Conflict, Duplicate, OutdatedDocument, Inconsistency, SemanticSimilarity, ConflictType, DuplicateType, OutdatedType, InconsistencyType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueReport {
    pub issues: Vec<Issue>,
    pub summary: IssueSummary,
    pub metadata: ReportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub issue_type: IssueType,
    pub severity: Severity,
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub affected_documents: Vec<String>,
    pub evidence: Vec<String>,
    pub recommendations: Vec<String>,
    pub explainability: ExplainabilityScore,
    pub auto_fixable: bool,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    Conflict,
    Duplicate,
    Outdated,
    Inconsistency,
    SemanticSimilarity,
    StructuralIssue,
    ComplianceIssue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum Priority {
    Immediate,
    High,
    Medium,
    Low,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainabilityScore {
    pub overall: f32,
    pub evidence_strength: f32,
    pub rule_clarity: f32,
    pub context_relevance: f32,
    pub confidence_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueSummary {
    pub total_issues: usize,
    pub critical_issues: usize,
    pub high_issues: usize,
    pub medium_issues: usize,
    pub low_issues: usize,
    pub info_issues: usize,
    pub auto_fixable_count: usize,
    pub severity_distribution: HashMap<String, usize>,
    pub type_distribution: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub generated_at: String,
    pub analyzer_version: String,
    pub documents_analyzed: usize,
    pub analysis_duration_ms: u64,
    pub confidence_threshold: f32,
}

pub struct IssueDetector {
    confidence_threshold: f32,
    severity_weights: HashMap<IssueType, Severity>,
    priority_weights: HashMap<Severity, Priority>,
}

impl IssueDetector {
    pub fn new() -> Self {
        let mut severity_weights = HashMap::new();
        severity_weights.insert(IssueType::Conflict, Severity::Critical);
        severity_weights.insert(IssueType::Duplicate, Severity::High);
        severity_weights.insert(IssueType::Outdated, Severity::Medium);
        severity_weights.insert(IssueType::Inconsistency, Severity::Medium);
        severity_weights.insert(IssueType::SemanticSimilarity, Severity::Low);
        severity_weights.insert(IssueType::StructuralIssue, Severity::Medium);
        severity_weights.insert(IssueType::ComplianceIssue, Severity::High);

        let mut priority_weights = HashMap::new();
        priority_weights.insert(Severity::Critical, Priority::Immediate);
        priority_weights.insert(Severity::High, Priority::High);
        priority_weights.insert(Severity::Medium, Priority::Medium);
        priority_weights.insert(Severity::Low, Priority::Low);
        priority_weights.insert(Severity::Info, Priority::Deferred);

        Self {
            confidence_threshold: 0.7,
            severity_weights,
            priority_weights,
        }
    }

    pub fn with_confidence_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    pub fn detect_issues(
        &self,
        conflicts: &[Conflict],
        duplicates: &[Duplicate],
        outdated: &[OutdatedDocument],
        inconsistencies: &[Inconsistency],
        semantic_similarities: &[SemanticSimilarity],
    ) -> IssueReport {
        let start_time = std::time::Instant::now();
        
        let mut issues = Vec::new();

        for (i, conflict) in conflicts.iter().enumerate() {
            if let Some(issue) = self.create_conflict_issue(conflict, i) {
                issues.push(issue);
            }
        }

        for (i, duplicate) in duplicates.iter().enumerate() {
            if let Some(issue) = self.create_duplicate_issue(duplicate, i) {
                issues.push(issue);
            }
        }

        for (i, outdated_doc) in outdated.iter().enumerate() {
            if let Some(issue) = self.create_outdated_issue(outdated_doc, i) {
                issues.push(issue);
            }
        }

        for (i, inconsistency) in inconsistencies.iter().enumerate() {
            if let Some(issue) = self.create_inconsistency_issue(inconsistency, i) {
                issues.push(issue);
            }
        }

        for (i, similarity) in semantic_similarities.iter().enumerate() {
            if let Some(issue) = self.create_semantic_similarity_issue(similarity, i) {
                issues.push(issue);
            }
        }

        issues.sort_by(|a, b| {
            b.severity.partial_cmp(&a.severity)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal))
        });

        let summary = self.create_summary(&issues);
        let metadata = ReportMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            analyzer_version: "1.0.0".to_string(),
            documents_analyzed: self.count_unique_documents(&issues),
            analysis_duration_ms: start_time.elapsed().as_millis() as u64,
            confidence_threshold: self.confidence_threshold,
        };

        IssueReport {
            issues,
            summary,
            metadata,
        }
    }

    fn create_conflict_issue(&self, conflict: &Conflict, index: usize) -> Option<Issue> {
        let confidence = self.calculate_confidence(conflict.severity, &conflict.evidence);
        
        if confidence < self.confidence_threshold {
            return None;
        }

        let severity = self.adjust_severity_by_confidence(
            self.severity_weights.get(&IssueType::Conflict).unwrap_or(&Severity::Critical).clone(),
            confidence
        );

        Some(Issue {
            id: format!("CONF_{:03}", index),
            issue_type: IssueType::Conflict,
            severity,
            priority: self.priority_weights.get(&severity).unwrap_or(&Priority::Immediate).clone(),
            title: format!("Conflict between {} and {}", conflict.doc1_id, conflict.doc2_id),
            description: conflict.description.clone(),
            affected_documents: vec![conflict.doc1_id.clone(), conflict.doc2_id.clone()],
            evidence: conflict.evidence.clone(),
            recommendations: self.generate_conflict_recommendations(conflict),
            explainability: self.calculate_explainability(conflict.severity, &conflict.evidence),
            auto_fixable: self.is_conflict_auto_fixable(conflict),
            confidence,
        })
    }

    fn create_duplicate_issue(&self, duplicate: &Duplicate, index: usize) -> Option<Issue> {
        let confidence = duplicate.similarity_score;
        
        if confidence < self.confidence_threshold {
            return None;
        }

        let severity = match duplicate.duplicate_type {
            DuplicateType::Identical => Severity::High,
            DuplicateType::NearIdentical => Severity::High,
            DuplicateType::SubstantiallySimilar => Severity::Medium,
            DuplicateType::StructuralSimilarity => Severity::Low,
        };

        let adjusted_severity = self.adjust_severity_by_confidence(severity, confidence);

        Some(Issue {
            id: format!("DUP_{:03}", index),
            issue_type: IssueType::Duplicate,
            severity: adjusted_severity,
            priority: self.priority_weights.get(&adjusted_severity).unwrap_or(&Priority::High).clone(),
            title: format!("Duplicate content: {} and {}", duplicate.doc1_id, duplicate.doc2_id),
            description: format!("Documents are {:.1}% similar", duplicate.similarity_score * 100.0),
            affected_documents: vec![duplicate.doc1_id.clone(), duplicate.doc2_id.clone()],
            evidence: vec![format!("Similarity score: {:.3}", duplicate.similarity_score)],
            recommendations: self.generate_duplicate_recommendations(duplicate),
            explainability: self.calculate_explainability(duplicate.similarity_score, &[]),
            auto_fixable: matches!(duplicate.duplicate_type, DuplicateType::Identical | DuplicateType::NearIdentical),
            confidence,
        })
    }

    fn create_outdated_issue(&self, outdated: &OutdatedDocument, index: usize) -> Option<Issue> {
        let confidence = 0.9;
        let severity = match outdated.outdated_type {
            OutdatedType::Repealed => Severity::High,
            OutdatedType::Expired => Severity::Medium,
            OutdatedType::Superseded => Severity::Medium,
            OutdatedType::InconsistentWithCurrentLaw => Severity::High,
        };

        Some(Issue {
            id: format!("OUT_{:03}", index),
            issue_type: IssueType::Outdated,
            severity,
            priority: self.priority_weights.get(&severity).unwrap_or(&Priority::Medium).clone(),
            title: format!("Outdated document: {}", outdated.doc_id),
            description: outdated.reason.clone(),
            affected_documents: vec![outdated.doc_id.clone()],
            evidence: vec![outdated.reason.clone()],
            recommendations: self.generate_outdated_recommendations(outdated),
            explainability: self.calculate_explainability(0.9, &[outdated.reason.clone()]),
            auto_fixable: false,
            confidence,
        })
    }

    fn create_inconsistency_issue(&self, inconsistency: &Inconsistency, index: usize) -> Option<Issue> {
        let confidence = 0.8;
        let severity = match inconsistency.inconsistency_type {
            InconsistencyType::InternalContradiction => Severity::High,
            InconsistencyType::MissingReferences => Severity::Medium,
            InconsistencyType::InvalidCrossReferences => Severity::Medium,
            InconsistencyType::FormattingIssues => Severity::Low,
            InconsistencyType::NumberingIssues => Severity::Low,
        };

        Some(Issue {
            id: format!("INC_{:03}", index),
            issue_type: IssueType::Inconsistency,
            severity,
            priority: self.priority_weights.get(&severity).unwrap_or(&Priority::Medium).clone(),
            title: format!("Inconsistency in document: {}", inconsistency.doc_id),
            description: inconsistency.description.clone(),
            affected_documents: vec![inconsistency.doc_id.clone()],
            evidence: inconsistency.affected_sections.clone(),
            recommendations: self.generate_inconsistency_recommendations(inconsistency),
            explainability: self.calculate_explainability(0.8, &inconsistency.affected_sections),
            auto_fixable: matches!(inconsistency.inconsistency_type, InconsistencyType::FormattingIssues | InconsistencyType::NumberingIssues),
            confidence,
        })
    }

    fn create_semantic_similarity_issue(&self, similarity: &SemanticSimilarity, index: usize) -> Option<Issue> {
        if similarity.similarity_score < 0.8 {
            return None;
        }

        let severity = Severity::Low;
        let confidence = similarity.similarity_score;

        Some(Issue {
            id: format!("SEM_{:03}", index),
            issue_type: IssueType::SemanticSimilarity,
            severity,
            priority: Priority::Low,
            title: format!("Semantically similar documents: {} and {}", similarity.doc1_id, similarity.doc2_id),
            description: format!("Documents are {:.1}% semantically similar", similarity.similarity_score * 100.0),
            affected_documents: vec![similarity.doc1_id.clone(), similarity.doc2_id.clone()],
            evidence: vec![format!("Semantic similarity: {:.3}", similarity.similarity_score)],
            recommendations: self.generate_semantic_similarity_recommendations(similarity),
            explainability: self.calculate_explainability(similarity.similarity_score, &[]),
            auto_fixable: false,
            confidence,
        })
    }

    fn calculate_confidence(&self, base_score: f32, evidence: &[String]) -> f32 {
        let evidence_factor = if evidence.is_empty() { 0.5 } else { 0.9 };
        (base_score + evidence_factor) / 2.0
    }

    fn adjust_severity_by_confidence(&self, base_severity: Severity, confidence: f32) -> Severity {
        if confidence < 0.5 {
            match base_severity {
                Severity::Critical => Severity::High,
                Severity::High => Severity::Medium,
                Severity::Medium => Severity::Low,
                Severity::Low => Severity::Info,
                Severity::Info => Severity::Info,
            }
        } else {
            base_severity
        }
    }

    fn calculate_explainability(&self, confidence: f32, evidence: &[String]) -> ExplainabilityScore {
        let evidence_strength = if evidence.is_empty() { 0.3 } else { 0.8 };
        
        ExplainabilityScore {
            overall: (confidence + evidence_strength) / 2.0,
            evidence_strength,
            rule_clarity: 0.9,
            context_relevance: 0.8,
            confidence_level: confidence,
        }
    }

    fn is_conflict_auto_fixable(&self, conflict: &Conflict) -> bool {
        matches!(conflict.conflict_type, ConflictType::AmendmentConflict | ConflictType::TimelineConflict)
    }

    fn generate_conflict_recommendations(&self, conflict: &Conflict) -> Vec<String> {
        match conflict.conflict_type {
            ConflictType::RepealConflict => vec![
                "Update target document status to reflect repeal".to_string(),
                "Add reconciliation notes to both documents".to_string(),
            ],
            ConflictType::AmendmentConflict => {
                vec![
                    "Verify document dates and amendment sequence".to_string(),
                    "Consider document hierarchy for precedence".to_string(),
                ]
            },
            ConflictType::ContradictoryRegulations => {
                vec![
                    "Manual review required to resolve contradictions".to_string(),
                    "Consider creating a superseding document".to_string(),
                ]
            },
            _ => vec!["Further investigation required".to_string()],
        }
    }

    fn generate_duplicate_recommendations(&self, duplicate: &Duplicate) -> Vec<String> {
        match duplicate.duplicate_type {
            DuplicateType::Identical => vec![
                "Consider removing one of the identical documents".to_string(),
                "Merge document metadata if both are needed".to_string(),
            ],
            DuplicateType::NearIdentical => vec![
                "Review differences between documents".to_string(),
                "Consolidate if differences are minor".to_string(),
            ],
            DuplicateType::SubstantiallySimilar => vec![
                "Review for potential consolidation opportunities".to_string(),
                "Consider cross-referencing instead of duplication".to_string(),
            ],
            DuplicateType::StructuralSimilarity => vec![
                "Review for template standardization opportunities".to_string(),
                "Consider document structure guidelines".to_string(),
            ],
        }
    }

    fn generate_outdated_recommendations(&self, outdated: &OutdatedDocument) -> Vec<String> {
        let mut recommendations = vec![
            "Archive outdated document".to_string(),
            "Update references to point to current documents".to_string(),
        ];

        if !outdated.replacement_suggestions.is_empty() {
            recommendations.push(format!("Consider using replacement documents: {:?}", outdated.replacement_suggestions));
        }

        recommendations
    }

    fn generate_inconsistency_recommendations(&self, inconsistency: &Inconsistency) -> Vec<String> {
        match inconsistency.inconsistency_type {
            InconsistencyType::InternalContradiction => vec![
                "Review document for logical consistency".to_string(),
                "Consider document revision".to_string(),
            ],
            InconsistencyType::MissingReferences => vec![
                "Update or remove invalid references".to_string(),
                "Verify all document links".to_string(),
            ],
            InconsistencyType::InvalidCrossReferences => vec![
                "Fix cross-reference formatting".to_string(),
                "Validate all referenced document IDs".to_string(),
            ],
            InconsistencyType::FormattingIssues => vec![
                "Apply standard formatting template".to_string(),
                "Review document structure guidelines".to_string(),
            ],
            InconsistencyType::NumberingIssues => vec![
                "Renumber articles sequentially".to_string(),
                "Update internal references".to_string(),
            ],
        }
    }

    fn generate_semantic_similarity_recommendations(&self, similarity: &SemanticSimilarity) -> Vec<String> {
        vec![
            "Review for potential consolidation".to_string(),
            "Consider cross-referencing to avoid redundancy".to_string(),
            "Evaluate if both documents serve distinct purposes".to_string(),
        ]
    }

    fn create_summary(&self, issues: &[Issue]) -> IssueSummary {
        let mut severity_distribution = HashMap::new();
        let mut type_distribution = HashMap::new();
        let mut critical = 0;
        let mut high = 0;
        let mut medium = 0;
        let mut low = 0;
        let mut info = 0;
        let mut auto_fixable = 0;

        for issue in issues {
            *severity_distribution.entry(format!("{:?}", issue.severity)).or_insert(0) += 1;
            *type_distribution.entry(format!("{:?}", issue.issue_type)).or_insert(0) += 1;

            match issue.severity {
                Severity::Critical => critical += 1,
                Severity::High => high += 1,
                Severity::Medium => medium += 1,
                Severity::Low => low += 1,
                Severity::Info => info += 1,
            }

            if issue.auto_fixable {
                auto_fixable += 1;
            }
        }

        IssueSummary {
            total_issues: issues.len(),
            critical_issues: critical,
            high_issues: high,
            medium_issues: medium,
            low_issues: low,
            info_issues: info,
            auto_fixable_count: auto_fixable,
            severity_distribution,
            type_distribution,
        }
    }

    fn count_unique_documents(&self, issues: &[Issue]) -> usize {
        let mut unique_docs = std::collections::HashSet::new();
        for issue in issues {
            for doc_id in &issue.affected_documents {
                unique_docs.insert(doc_id.clone());
            }
        }
        unique_docs.len()
    }
}

impl Default for IssueDetector {
    fn default() -> Self {
        Self::new()
    }
}
