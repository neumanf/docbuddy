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
    title: String,
    description: String,
}

#[tauri::command]
pub async fn search(site: &str, search: &str) -> Result<Vec<SearchResponse>, RequestError> {
    let encoded_search = encode(search);
    let url = format!("https://html.duckduckgo.com/html/?q={encoded_search}+site%3A{site}");
    let html = reqwest::get(url.clone()).await?.text().await?;

    let document = Html::parse_document(&html);

    let results_selector = Selector::parse(".result__body").unwrap();
    let title_selector = Selector::parse(".result__title").unwrap();
    let description_selector = Selector::parse(".result__snippet").unwrap();
    let url_selector = Selector::parse(".result__url").unwrap();

    let mut response: Vec<SearchResponse> = vec![];
    let mut items_count = 0;

    for result in document.select(&results_selector) {
        if items_count >= 5 {
            break;
        }

        let mut description = String::new();

        for idk in result.select(&description_selector) {
            description += &idk.text().collect::<Vec<_>>().concat();
        }

        let mut title = String::new();

        for idk in result.select(&title_selector) {
            title += &idk.text().collect::<Vec<_>>().concat();
        }

        let mut href = String::new();

        for idk in result.select(&url_selector) {
            href = "https://".to_string();
            href.push_str(&idk.text().collect::<Vec<_>>().concat().trim());
        }

        if !href.is_empty()
            && !title.is_empty()
            && !title.contains("EOF")
            && !title.contains("Ad Viewing")
        {
            response.push(SearchResponse {
                href,
                title,
                description,
            });

            items_count = items_count + 1;
        }
    }

    return Ok(response);
}
