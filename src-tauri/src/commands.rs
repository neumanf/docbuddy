use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use urlencoding::encode;

#[derive(Serialize)]
pub enum RequestError {
    NetworkError(String),
}

impl From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        RequestError::NetworkError(error.to_string())
    }
}

#[derive(Serialize)]
pub struct SearchResponse {
    href: String,
    title: String
}

#[tauri::command]
pub async fn search(search: &str) -> Result<SearchResponse, RequestError> {
    let encoded_search = encode(search);
    let website = "tailwindcss.com";
    let url = format!("https://html.duckduckgo.com/html/?q={encoded_search}+site%3A{website}");
    let html = reqwest::get(url)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);
    let links_selector = Selector::parse(".result__title").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    for link in document.select(&links_selector) {
        for a in link.select(&a_selector) {
            let href_attr = a.value().attr("href");

            if let Some(href) = href_attr.filter(|t| !t.is_empty()) {
                let title = a.text().collect::<Vec<_>>().concat();
                let href = href.to_string();

                return Ok(SearchResponse {
                    title,
                    href
                })
            }
        }
    }

    Err(RequestError::NetworkError("No results found".to_string()))
}
