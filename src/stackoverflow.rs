use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.stackexchange.com/2.3";

#[derive(Deserialize)]
struct ApiResponse {
    items: Vec<Question>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Question {
    pub question_id: u32,
    pub title: String,
    pub link: String,
    pub is_answered: bool,
    pub answer_count: u32,
}

pub async fn fetch_nearprotocol_questions(client: &Client) -> Result<Vec<Question>, reqwest::Error> {
    let params: HashMap<&str, &str> = [
        ("site", "stackoverflow"),
        ("tagged", "nearprotocol"),
        ("sort", "activity"),
        ("order", "desc"),
    ]
    .into_iter()
    .collect();

    let response = client
        .get(&format!("{}/questions", BASE_URL))
        .query(&params)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;
    
    Ok(response.items)
}
