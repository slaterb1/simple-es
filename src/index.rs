use reqwest::StatusCode;
use serde::Deserialize;
use std::fmt;

use crate::client::EsClient;
use crate::utils::serialize_response;
use crate::errors::ESClientCreateIndexFail;

#[derive(Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::{EsIndexCreateSuccess, create_index_req};
    use crate::client::EsClient;
    use mockito::mock;
    use tokio::runtime::Runtime;

    #[test]
    fn successfully_create_index() {
        let mut rt = Runtime::new().unwrap();
        let _client_mock = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "name": "DbU-kT2",
                "cluster_name": "docker-cluster",
                "cluster_uuid": "HjwlCaVKQo2766zcX_l7DQ",
                "version": { 
                    "number": "6.8.6",
                    "build_flavor": "default",
                    "build_type": "docker",
                    "build_hash": "3d9f765",
                    "build_date": "2019-12-13T17:11:52.013738Z",
                    "build_snapshot": false,
                    "lucene_version": "7.7.2",
                    "minimum_wire_compatibility_version": "5.6.0",
                    "minimum_index_compatibility_version": "5.0.0"
                },
                "tagline": "You Know, for Search" 
            }"#)
            .create();

        let _index_mock = mock("PUT", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "acknowledged": true,
                "shards_acknowledged": true,
                "index": "test"
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = create_index_req(&client, "test");
        let res = rt.block_on(res);
        let expected_res = EsIndexCreateSuccess {
            acknowledged: true,
            shards_acknowledged: true,
            index: "test".to_owned(),
        };
        assert_eq!(res.unwrap(), expected_res);
    }
}
