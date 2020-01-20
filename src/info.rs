use reqwest;
use serde::Deserialize;

use crate::client::EsClient;

#[derive(Deserialize, Debug)]
pub struct EsInfo {
    name: String,
    cluster_name: String,
    cluster_uuid: String,
    version: Version,
    tagline: String,
}

#[derive(Deserialize, Debug)]
struct Version {
    number: String,
    build_flavor: String,
    build_type: String,
    build_hash: String,
    build_date: String,
    build_snapshot: bool,
    lucene_version: String,
    minimum_wire_compatibility_version: String,
    minimum_index_compatibility_version: String,
}

impl EsInfo {
    pub fn get_version_string(&self) -> String {
        self.version.number.clone()
    }
}

pub async fn es_info_req(client: &EsClient) -> reqwest::Result<EsInfo> {
    let res = client.get()
        .send()
        .await?
        .json::<EsInfo>()
        .await?;
    Ok(res)
}

