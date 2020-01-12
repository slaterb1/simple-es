use reqwest::StatusCode;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

use crate::client::EsClient;
use crate::utils::serialize_response;

#[derive(Deserialize, Debug)]
pub struct EsIndexCreateSuccess {
    acknowledged: bool,
    shards_acknowledged: bool,
    index: String,
}

#[derive(Deserialize, Debug)]
struct EsIndexCreateFail {
    error: EsIndexError,
    status: u16,
}

impl fmt::Display for EsIndexCreateSuccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "acknowledged: {}, shards_acknowledged: {}, index: {}", self.acknowledged, self.shards_acknowledged, self.index)
    }
}

impl fmt::Display for EsIndexCreateFail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {:?}, status: {}", self.error, self.status)
    }
}

impl Error for EsIndexCreateFail {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Deserialize, Debug)]
struct EsIndexError {
    root_cause: Vec<IndexCreateFailMetadata>,
}

#[derive(Deserialize, Debug)]
struct IndexCreateFailMetadata {
    #[serde(rename = "type")]
    error_type: String,
    reason: String,
    index_uuid: String,
    index: String,
}

pub async fn create_index_req(client: &EsClient, index: &str) -> Result<EsIndexCreateSuccess, Box<dyn std::error::Error>> {
    let res = client.put(Some(index), None)
        .send()
        .await?;

    let res = match res.status() {
        StatusCode::OK => {
            let text = res.text().await?;
            let data = serialize_response::<EsIndexCreateSuccess>(&text)?;
            data
        },
        StatusCode::BAD_REQUEST => { 
            let text = res.text().await?;
            let data = serialize_response::<EsIndexCreateFail>(&text)?;
            return Err(Box::new(data));
        },
        _ => panic!("Request failed in an unexpected way..."),
    };
    Ok(res)
}
