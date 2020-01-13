use reqwest;
use reqwest::StatusCode;
use serde_json::Value;
use serde::Deserialize;

use crate::client::EsClient;
use crate::utils::serialize_response;
use crate::errors::ESClientSearchFail;

#[derive(Deserialize, Debug)]
pub struct EsSearchResponse<T> {
    took: u16,
    timed_out: bool,
    #[serde(rename = "_shards")]
    shards: ShardResults,
    hits: HitResults<T>,
}

#[derive(Deserialize, Debug)]
struct ShardResults {
    total: u16,
    successful: u16,
    skipped: u16,
    failed: u16,
}

#[derive(Deserialize, Debug)]
struct HitResults<T> {
    hits: Vec<Option<T>>,
    total: u16,
    max_score: Option<f32>,
}



pub async fn search_req<T>(client: &EsClient, index: &str, doc_type: Option<&str>, query: Value) -> Result<EsSearchResponse<T>, Box<dyn std::error::Error>>
    where for<'de> T: Deserialize<'de>
{
    let res = client.post(Some(index), doc_type, Some("_search"))
        .json(&query)
        .send()
        .await?;

    let res = match res.status() {
        StatusCode::OK => {
            let text = res.text().await?;
            let data = serialize_response::<EsSearchResponse<T>>(&text)?;
            data
        },
        StatusCode::BAD_REQUEST => { 
            let text = res.text().await?;
            let data = serialize_response::<ESClientSearchFail>(&text)?;
            return Err(Box::new(data));
        },
        _ => panic!("Request failed in an unexpected way..."),
    };
    Ok(res)
}

