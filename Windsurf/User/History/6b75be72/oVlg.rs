use std::collections::HashMap;
use regex::Regex;
use crate::data::parser::{LawDocument, LinkType};
use crate::graph::DocumentGraph;

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub conflicts: Vec<Conflict>,
    pub duplicates: Vec<Duplicate>,
    pub outdated: Vec<OutdatedDocument>,
    pub inconsistencies: Vec<Inconsistency>,
    pub semantic_similarities: Vec<SemanticSimilarity>,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub doc1_id: String,
    pub doc2_id: String,
    pub conflict_type: ConflictType,
    pub description: String,
    pub severity: f32,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ConflictType {
    ContradictoryRegulations,
    OverlappingJurisdiction,
    RepealConflict,
    AmendmentConflict,
    TimelineConflict,
}

#[derive(Debug, Clone)]
pub struct Duplicate {
    pub doc1_id: String,
    pub doc2_id: String,
    pub similarity_score: f32,
    pub duplicate_type: DuplicateType,
    pub overlapping_sections: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum DuplicateType {
    Identical,
    NearIdentical,
    SubstantiallySimilar,
    StructuralSimilarity,
}

#[derive(Debug, Clone)]
pub struct OutdatedDocument {
    pub doc_id: String,
    pub outdated_type: OutdatedType,
    pub reason: String,
    pub replacement_suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum OutdatedType {
    Expired,
    Superseded,
    Repealed,
    InconsistentWithCurrentLaw,
}

#[derive(Debug, Clone)]
pub struct Inconsistency {
    pub doc_id: String,
    pub inconsistency_type: InconsistencyType,
    pub description: String,
    pub affected_sections: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum InconsistencyType {
    InternalContradiction,
    MissingReferences,
    InvalidCrossReferences,
    FormattingIssues,
    NumberingIssues,
}

#[derive(Debug, Clone)]
pub struct SemanticSimilarity {
    pub doc1_id: String,
    pub doc2_id: String,
    pub similarity_score: f32,
    pub similar_sections: Vec<(String, String)>,
}

pub struct DocumentAnalyzer {
    conflict_patterns: HashMap<ConflictType, Vec<Regex>>,
}

impl DocumentAnalyzer {
    pub fn new() -> Self {
        let mut conflict_patterns = HashMap::new();

        conflict_patterns.insert(
            ConflictType::ContradictoryRegulations,
            vec![
                Regex::new(r"(?i)(запрещает|не допускается).*?(разрешает|допускается)").unwrap(),
                Regex::new(r"(?i)(обязан|должен).*?(не обязан|не должен)").unwrap(),
            ],
        );

        conflict_patterns.insert(
            ConflictType::OverlappingJurisdiction,
            vec![Regex::new(r"(?i)(компетенция|полномочия).*?(орган|ведомство)").unwrap()],
        );

        Self {
            conflict_patterns,
        }
    }

    pub fn analyze_documents(
        &self,
        documents: &[LawDocument],
        graph: &DocumentGraph,
    ) -> AnalysisResult {
        let conflicts = self.detect_conflicts(documents, graph);
        let duplicates = self.detect_duplicates(documents);
        let outdated = self.detect_outdated(documents, graph);
        let inconsistencies = self.detect_inconsistencies(documents);

        let semantic_similarities = Vec::new();

        AnalysisResult {
            conflicts,
            duplicates,
            outdated,
            inconsistencies,
            semantic_similarities,
        }
    }

    fn detect_conflicts(&self, documents: &[LawDocument], graph: &DocumentGraph) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        for doc in documents {
            for link in &doc.links {
                if let LinkType::Repeal = link.link_type {
                    if let Some(target_doc) = documents.iter().find(|d| d.id == link.target_id) {
                        if target_doc.status.as_deref() == Some("действует")
                            || target_doc.status.as_deref() == Some("active")
                        {
                            conflicts.push(Conflict {
                                doc1_id: doc.id.clone(),
                                doc2_id: link.target_id.clone(),
                                conflict_type: ConflictType::RepealConflict,
                                description: format!(
                                    "Document {} claims to repeal {} but target is still active",
                                    doc.id, link.target_id
                                ),
                                severity: 0.9,
                                evidence: vec![link.context.clone()],
                            });
                        }
                    }
                }
            }
        }

        for doc in documents {
            for link in &doc.links {
                if let LinkType::Amendment = link.link_type {
                    if let Some(target_doc) = documents.iter().find(|d| d.id == link.target_id) {
                        if let Some(target_date) = &target_doc.date {
                            if let Some(doc_date) = &doc.date {
                                if doc_date < target_date {
                                    conflicts.push(Conflict {
                                        doc1_id: doc.id.clone(),
                                        doc2_id: link.target_id.clone(),
                                        conflict_type: ConflictType::AmendmentConflict,
                                        description: format!(
                                            "Document {} amends {} but is older than target",
                                            doc.id, link.target_id
                                        ),
                                        severity: 0.7,
                                        evidence: vec![format!(
                                            "Doc date: {}, Target date: {}",
                                            doc_date, target_date
                                        )],
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        for (i, doc1) in documents.iter().enumerate() {
            for doc2 in documents.iter().skip(i + 1) {
                if let Some(similar_sections) = self.find_textual_conflicts(doc1, doc2) {
                    conflicts.push(Conflict {
                        doc1_id: doc1.id.clone(),
                        doc2_id: doc2.id.clone(),
                        conflict_type: ConflictType::ContradictoryRegulations,
                        description: "Contradictory regulations found".to_string(),
                        severity: 0.6,
                        evidence: similar_sections,
                    });
                }
            }
        }

        conflicts
    }

    fn find_textual_conflicts(
        &self,
        doc1: &LawDocument,
        doc2: &LawDocument,
    ) -> Option<Vec<String>> {
        let combined_text = format!("{} {}", doc1.text, doc2.text);

        for (conflict_type, patterns) in &self.conflict_patterns {
            for pattern in patterns {
                if pattern.is_match(&combined_text) {
                    return Some(vec![format!("Pattern match: {:?}", conflict_type)]);
                }
            }
        }

        None
    }

    fn detect_duplicates(&self, documents: &[LawDocument]) -> Vec<Duplicate> {
        let mut duplicates = Vec::new();

        for (i, doc1) in documents.iter().enumerate() {
            for doc2 in documents.iter().skip(i + 1) {
                let similarity = self.calculate_text_similarity(&doc1.text, &doc2.text);

                if similarity > 0.8 {
                    duplicates.push(Duplicate {
                        doc1_id: doc1.id.clone(),
                        doc2_id: doc2.id.clone(),
                        similarity_score: similarity,
                        duplicate_type: if similarity > 0.95 {
                            DuplicateType::Identical
                        } else if similarity > 0.9 {
                            DuplicateType::NearIdentical
                        } else {
                            DuplicateType::SubstantiallySimilar
                        },
                        overlapping_sections: vec![],
                    });
                }
            }
        }

        duplicates
    }

    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f32 {
        let words1: std::collections::HashSet<&str> = text1
            .split_whitespace()
            .map(|w| {
                w.to_lowercase()
                    .trim_matches(|c: char| !c.is_alphanumeric())
            })
            .collect();

        let words2: std::collections::HashSet<&str> = text2
            .split_whitespace()
            .map(|w| {
                w.to_lowercase()
                    .trim_matches(|c: char| !c.is_alphanumeric())
            })
            .collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    fn detect_outdated(
        &self,
        documents: &[LawDocument],
        graph: &DocumentGraph,
    ) -> Vec<OutdatedDocument> {
        let mut outdated = Vec::new();

        for doc in documents {
            if let Some(status) = &doc.status {
                if status.contains("утратил") || status.contains("отменен") {
                    outdated.push(OutdatedDocument {
                        doc_id: doc.id.clone(),
                        outdated_type: OutdatedType::Repealed,
                        reason: format!("Document status: {}", status),
                        replacement_suggestions: self.find_replacement_suggestions(doc, graph),
                    });
                }
            }

            if let Some(date_str) = &doc.date {
                if let Ok(parsed_date) = self.parse_date(date_str) {
                    if self.is_document_expired(&parsed_date) {
                        outdated.push(OutdatedDocument {
                            doc_id: doc.id.clone(),
                            outdated_type: OutdatedType::Expired,
                            reason: format!("Document expired on {}", date_str),
                            replacement_suggestions: self.find_replacement_suggestions(doc, graph),
                        });
                    }
                }
            }

            let dependents = graph.find_dependents(&doc.id);
            for dependent in dependents {
                if let Some(dep_doc) = documents.iter().find(|d| d.id == dependent.id) {
                    for link in &dep_doc.links {
                        if link.target_id == doc.id && matches!(link.link_type, LinkType::Amendment)
                        {
                            outdated.push(OutdatedDocument {
                                doc_id: doc.id.clone(),
                                outdated_type: OutdatedType::Superseded,
                                reason: format!("Superseded by document {}", dep_doc.id),
                                replacement_suggestions: vec![dep_doc.id.clone()],
                            });
                        }
                    }
                }
            }
        }

        outdated
    }

    fn detect_inconsistencies(&self, documents: &[LawDocument]) -> Vec<Inconsistency> {
        let mut inconsistencies = Vec::new();

        for doc in documents {
            if let Some(evidence) = self.check_internal_contradictions(doc) {
                inconsistencies.push(Inconsistency {
                    doc_id: doc.id.clone(),
                    inconsistency_type: InconsistencyType::InternalContradiction,
                    description: "Internal contradiction found".to_string(),
                    affected_sections: evidence,
                });
            }

            let missing_refs = self.check_missing_references(doc);
            if !missing_refs.is_empty() {
                inconsistencies.push(Inconsistency {
                    doc_id: doc.id.clone(),
                    inconsistency_type: InconsistencyType::MissingReferences,
                    description: "Missing document references".to_string(),
                    affected_sections: missing_refs,
                });
            }

            if let Some(numbering_issues) = self.check_article_numbering(doc) {
                inconsistencies.push(Inconsistency {
                    doc_id: doc.id.clone(),
                    inconsistency_type: InconsistencyType::NumberingIssues,
                    description: "Article numbering inconsistencies".to_string(),
                    affected_sections: numbering_issues,
                });
            }
        }

        inconsistencies
    }

    fn check_internal_contradictions(&self, doc: &LawDocument) -> Option<Vec<String>> {
        let text_lower = doc.text.to_lowercase();

        if text_lower.contains("запрещает") && text_lower.contains("разрешает") {
            Some(vec![
                "Potential contradiction between prohibitions and permissions".to_string(),
            ])
        } else if text_lower.contains("обязан") && text_lower.contains("не обязан") {
            Some(vec!["Potential contradiction in obligations".to_string()])
        } else {
            None
        }
    }

    fn check_missing_references(&self, doc: &LawDocument) -> Vec<String> {
        let mut missing = Vec::new();

        for link in &doc.links {
            if link.target_id.len() < 3 {
                missing.push(format!("Suspicious reference: {}", link.target_id));
            }
        }

        missing
    }

    fn check_article_numbering(&self, doc: &LawDocument) -> Option<Vec<String>> {
        let mut issues = Vec::new();
        let mut prev_number = 0;

        for article in &doc.articles {
            if let Ok(num) = article.number.parse::<u32>() {
                if num <= prev_number {
                    issues.push(format!(
                        "Article numbering issue: {} after {}",
                        article.number, prev_number
                    ));
                }
                prev_number = num;
            }
        }

        if issues.is_empty() {
            None
        } else {
            Some(issues)
        }
    }

    fn find_replacement_suggestions(
        &self,
        doc: &LawDocument,
        graph: &DocumentGraph,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        let dependents = graph.find_dependents(&doc.id);
        for dependent in dependents {
            suggestions.push(dependent.id.clone());
        }

        suggestions
    }

    fn parse_date(&self, date_str: &str) -> Result<chrono::NaiveDate, Box<dyn std::error::Error>> {
        let formats = ["%d.%m.%Y", "%Y-%m-%d", "%d/%m/%Y"];

        for format in &formats {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, format) {
                return Ok(date);
            }
        }

        Err("Unable to parse date".into())
    }

    fn is_document_expired(&self, date: &chrono::NaiveDate) -> bool {
        let today = chrono::Utc::now().date_naive();
        today > *date
    }
}

impl Default for DocumentAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
