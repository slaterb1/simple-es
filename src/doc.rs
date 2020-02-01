use reqwest;
use reqwest::StatusCode;

use serde::{ Deserialize, Serialize };
use crate::client::EsClient;
use crate::utils::serialize_response;

#[derive(Deserialize, Debug, PartialEq)]
pub struct EsIndexDocResponse {
    #[serde(rename = "_shards")]
    shards: ShardResults,
    #[serde(rename = "_index")]
    index: String,
    #[serde(rename = "_type")]
    doc_type: String,
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_version")]
    version: u32,
    #[serde(rename = "_seq_no")]
    seq_no: u32,
    #[serde(rename = "_primary_term")]
    primary_term: u32,
    result: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ShardResults {
    total: u16,
    successful: u16,
    failed: u16,
}

pub async fn index_doc_req<T: Serialize>(
    client: &EsClient,
    index: &str,
    doc_type: Option<&str>,
    id: Option<&str>,
    operation: Option<&str>,
    data: T
    ) -> Result<EsIndexDocResponse, Box<dyn std::error::Error>> 
{
    let res = if let Some(id) = id {
        client.put_doc(index, doc_type, id, operation)
            .json(&data)
            .send()
            .await?
    } else {
        client.post_doc(index, doc_type)
            .json(&data)
            .send()
            .await?
    };
    println!("res: {:?}", res);

    let res = match res.status() {
        StatusCode::OK => {
            let text = res.text().await?;
            let data = serialize_response::<EsIndexDocResponse>(&text)?;
            data
        },
        StatusCode::CREATED => {
            let text = res.text().await?;
            let data = serialize_response::<EsIndexDocResponse>(&text)?;
            data
        },
        StatusCode::BAD_REQUEST => {
            let text = res.text().await?;
            println!("text: {:?}", text);
            panic!("Failed");
        },
        _ => panic!("Request failed in an unexpected way..."),
    };
    Ok(res)
}

