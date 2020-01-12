use reqwest;
use serde_json::json;
use serde::Deserialize;

use crate::client::EsClient;

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

pub async fn search_req<T>(client: &EsClient) -> reqwest::Result<EsSearchResponse<T>>
    where for<'de> T: Deserialize<'de>
{
    let res = client.post(Some("test"), None, Some("_search"))
        .json(
            &json!({
                "query": {
                    "match_all": {}
                }
            })
        )
        .send()
        .await?
        .json::<EsSearchResponse<T>>()
        .await?;

    Ok(res)
}

