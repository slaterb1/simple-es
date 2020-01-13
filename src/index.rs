use reqwest::StatusCode;
use serde::Deserialize;
use std::fmt;

use crate::client::EsClient;
use crate::utils::serialize_response;
use crate::errors::ESClientCreateIndexFail;

#[derive(Deserialize, Debug)]
pub struct EsIndexCreateSuccess {
    acknowledged: bool,
    shards_acknowledged: bool,
    index: String,
}

impl fmt::Display for EsIndexCreateSuccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "acknowledged: {}, shards_acknowledged: {}, index: {}", self.acknowledged, self.shards_acknowledged, self.index)
    }
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
            let data = serialize_response::<ESClientCreateIndexFail>(&text)?;
            return Err(Box::new(data));
        },
        _ => panic!("Request failed in an unexpected way..."),
    };
    Ok(res)
}
