use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawDocument {
    pub id: String,
    pub title: String,
    pub text: String,
    pub status: Option<String>,
    pub date: Option<String>,
    pub articles: Vec<Article>,
    pub links: Vec<DocumentLink>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub number: String,
    pub title: String,
    pub content: String,
    pub links: Vec<DocumentLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentLink {
    pub target_id: String,
    pub target_title: String,
    pub link_type: LinkType,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkType {
    Reference,
    Amendment,
    Repeal,
    Implementation,
    Other,
}

pub async fn parse_raw_document(doc_url: &str) -> Result<LawDocument, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()?;

    let body = client
        .get(doc_url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);
    
    let id = extract_document_id(doc_url)?;
    let title = parse_title(&document);
    let status = parse_status(&document);
    let date = parse_date(&document);
    let text = parse_text_content(&document);
    let articles = parse_articles(&document);
    let links = parse_links(&document, &text);
    let metadata = parse_metadata(&document);

    Ok(LawDocument {
        id,
        title,
        text,
        status,
        date,
        articles,
        links,
        metadata,
    })
}

fn extract_document_id(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r"/([A-Z0-9]+)_?$")?;
    if let Some(captures) = re.captures(url) {
        Ok(captures[1].to_string())
    } else {
        Ok(format!("doc_{}", url.split('/').last().unwrap_or("unknown")))
    }
}

fn parse_status(document: &Html) -> Option<String> {
    let status_selector = Selector::parse("span.status, .document-status, .status").ok();
    status_selector
        .as_ref()
        .and_then(|sel| document.select(sel).next())
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
}

fn parse_date(document: &Html) -> Option<String> {
    let date_selector = Selector::parse(".document-date, .date, .publication-date").ok();
    date_selector
        .as_ref()
        .and_then(|sel| document.select(sel).next())
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
}

fn parse_text_content(document: &Html) -> String {
    let content_selector = Selector::parse("article, .content, .document-content").unwrap();

    document
        .select(&content_selector)
        .flat_map(|el| el.text())
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_articles(document: &Html) -> Vec<Article> {
    let mut articles = Vec::new();
    let article_selector = Selector::parse("h2, h3, .article, .section").unwrap();
    
    for element in document.select(&article_selector) {
        let text = element.text().collect::<String>();
        if let Some(article) = extract_article_info(&text) {
            articles.push(article);
        }
    }
    
    articles
}

fn extract_article_info(text: &str) -> Option<Article> {
    let re = Regex::new(r"(?:Статья|Article)\s*(\d+)[\.\:\s]*(.+?)(?=\n|$)").ok()?;
    
    if let Some(captures) = re.captures(text) {
        Some(Article {
            number: captures[1].to_string(),
            title: captures[2].trim().to_string(),
            content: text.to_string(),
            links: Vec::new(),
        })
    } else {
        None
    }
}

fn parse_links(document: &Html, text: &str) -> Vec<DocumentLink> {
    let mut links = Vec::new();
    
    // Parse HTML links
    let link_selector = Selector::parse("a[href]").unwrap();
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(link) = create_link_from_html(href, element.text().collect::<String>().trim()) {
                links.push(link);
            }
        }
    }
    
    // Parse text references
    let text_links = extract_text_references(text);
    links.extend(text_links);
    
    links
}

fn create_link_from_html(href: &str, text: &str) -> Option<DocumentLink> {
    if href.contains("/docs/") || href.contains("/rus/docs/") {
        let target_id = href.split('/').last()?.trim_end_matches('_').to_string();
        Some(DocumentLink {
            target_id,
            target_title: text.to_string(),
            link_type: determine_link_type(text),
            context: text.to_string(),
        })
    } else {
        None
    }
}

fn extract_text_references(text: &str) -> Vec<DocumentLink> {
    let mut links = Vec::new();
    let patterns = [
        r"(?:согласно|в соответствии с|согласно п\.|в соответствии со ст\.)\s*([A-Z0-9]+)",
        r"(?:Закон|Указ|Постановление)\s+№?\s*([A-Z0-9]+)",
        r"([A-Z]{1,4}\d{4,8})",
    ];
    
    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            for captures in re.captures_iter(text) {
                if let Some(target_id) = captures.get(1) {
                    links.push(DocumentLink {
                        target_id: target_id.as_str().to_string(),
                        target_title: target_id.as_str().to_string(),
                        link_type: LinkType::Reference,
                        context: captures.get(0)?.as_str().to_string(),
                    });
                }
            }
        }
    }
    
    links
}

fn determine_link_type(text: &str) -> LinkType {
    let text_lower = text.to_lowercase();
    if text_lower.contains("отмен") || text_lower.contains("утратил") {
        LinkType::Repeal
    } else if text_lower.contains("изменен") || text_lower.contains("дополня") {
        LinkType::Amendment
    } else if text_lower.contains("примен") || text_lower.contains("исполн") {
        LinkType::Implementation
    } else {
        LinkType::Reference
    }
}

fn parse_metadata(document: &Html) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    
    // Extract various metadata fields
    let meta_selectors = [
        ("document_type", ".document-type, .type"),
        ("department", ".department, .ministry"),
        ("registration_number", ".reg-number, .number"),
        ("keywords", ".keywords, .tags"),
    ];
    
    for (key, selector) in &meta_selectors {
        if let Ok(sel) = Selector::parse(selector) {
            if let Some(element) = document.select(&sel).next() {
                let value = element.text().collect::<String>().trim().to_string();
                if !value.is_empty() {
                    metadata.insert(key.to_string(), value);
                }
            }
        }
    }
    
    metadata
}

fn parse_title(document: &Html) -> String {
    if let Some(selector) = Selector::parse("h1").ok() {
        if let Some(el) = document.select(&selector).next() {
            let title = el.text().collect::<String>().trim().to_string();
            if !title.is_empty() && title.len() < 500 {
                return title;
            }
        }
    }

    if let Some(selector) =
        Selector::parse(".container_alpha.slogan, .container_alpha .slogan").ok()
    {
        if let Some(el) = document.select(&selector).next() {
            let title = el.text().collect::<String>().trim().to_string();
            if !title.is_empty() && title.len() < 500 {
                return title;
            }
        }
    }

    if let Some(selector) = Selector::parse("article h1, article h2, article .post_header").ok() {
        if let Some(el) = document.select(&selector).next() {
            let title = el.text().collect::<String>().trim().to_string();
            if !title.is_empty() && title.len() < 500 {
                return title;
            }
        }
    }

    "Без названия".to_string()
}
