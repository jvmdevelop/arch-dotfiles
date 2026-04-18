//! # Legis-Entropy Library
//! 
//! AI-powered legal analysis system for detecting contradictions, duplications, 
//! and outdated norms in legislative documents.
//! 
//! ## Features
//! 
//! - Parsing and analysis of regulatory legal documents
//! - Detection of contradictions and duplications  
//! - **Real ML similarity analysis** using sentence-transformers and cross-encoders
//! - Export and visualization of results
//! 
//! ## Example
//! 
//! ```rust
//! use legis_entropy::{LegalAnalyzer, DocumentParser};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let parser = DocumentParser::new();
//!     let analyzer = LegalAnalyzer::new("dummy_token", 0.5);
//!     
//!     let doc = parser.parse("https://example.com/law").await?;
//!     let results = analyzer.analyze(vec![doc]).await?;
//!     
//!     println!("Found {} similar pairs", results.similar_pairs.len());
//!     Ok(())
//! }
//! ```

pub mod analyzer;
pub mod errors;
pub mod hf;
pub mod local_ml;
pub mod models;
pub mod parser;

pub use analyzer::LegalAnalyzer;
pub use errors::{AppError, Result};
pub use hf::EmbeddingService;
pub use local_ml::LocalMLService;
pub use models::{AnalysisResult, DocumentPair, LegalDocument};
pub use parser::DocumentParser;
