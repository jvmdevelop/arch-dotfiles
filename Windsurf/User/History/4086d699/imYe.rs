use crate::errors::{AppError, Result};
use crate::models::LegalDocument;
use reqwest::Client;
use scraper::{Html, Selector};
use tracing::info;

pub struct DocumentParser {
    client: Client,
}

impl DocumentParser {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("LegisEntropyBot/1.0 (Legal Analysis Bot)")
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn parse(&self, url: &str) -> Result<LegalDocument> {
        info!("Parsing document: {}", url);

        let body = self.fetch_html(url).await?;
        let document = Html::parse_document(&body);

        let id = self.extract_doc_id(url)?;
        let title = self.extract_title(&document);
        let text = self.extract_content(&document);
        let status = self.extract_status(&document);

        Ok(LegalDocument::new(id, title, text, status, url.to_string()))
    }

    async fn fetch_html(&self, url: &str) -> Result<String> {
        let response = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::Http(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            )));
        }

        let body = response.text().await?;
        
        if body.trim().is_empty() {
            return Err(AppError::Parsing("Empty response body".into()));
        }

        Ok(body)
    }

    fn extract_doc_id(&self, url: &str) -> Result<String> {
        url.split("/docs/")
            .nth(1)
            .ok_or_else(|| AppError::Parsing("Invalid URL format".into()))?
            .split("/")
            .next()
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::Parsing("Cannot extract doc ID".into()))
    }

    fn extract_title(&self, document: &Html) -> String {
        let selectors = [
            "h1",
            ".post_header",
            ".container_alpha.slogan",
            "article h1",
        ];

        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let title = element.text().collect::<String>().trim().to_string();
                    if !title.is_empty() && title.len() < 500 {
                        return title;
                    }
                }
            }
        }

        "Без названия".to_string()
    }

    fn extract_content(&self, document: &Html) -> String {
        let selectors = [
            "article",
            ".content", 
            "#main-content",
            ".module_npaSearch",
            ".document-content",
            "main",
        ];

        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                let content: String = document
                    .select(&selector)
                    .flat_map(|el| el.text())
                    .map(|t| t.trim())
                    .filter(|t| !t.is_empty() && t.len() > 3)
                    .collect::<Vec<_>>()
                    .join(" ");

                if !content.is_empty() && content.len() > 50 {
                    return content.trim().to_string();
                }
            }
        }

        document.root_element().text().collect::<String>()
    }

    fn extract_status(&self, document: &Html) -> Option<String> {
        if let Ok(selector) = Selector::parse("span.status") {
            document
                .select(&selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
        } else {
            None
        }
    }
}

impl Default for DocumentParser {
    fn default() -> Self {
        Self::new()
    }
}
