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
    let res = client.get(None)
        .send()
        .await?
        .json::<EsInfo>()
        .await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::{EsInfo, Version};

    #[test]
    fn test_get_version_string() {
        let info = EsInfo {
            name: "test".to_owned(),
            cluster_name: "test".to_owned(),
            cluster_uuid: "test".to_owned(),
            version: Version {
                number: "test".to_owned(),
                build_flavor: "test".to_owned(),
                build_type: "test".to_owned(),
                build_hash: "test".to_owned(),
                build_date: "test".to_owned(),
                build_snapshot: false,
                lucene_version: "test".to_owned(),
                minimum_wire_compatibility_version: "test".to_owned(),
                minimum_index_compatibility_version: "test".to_owned(),
            },
            tagline: "test".to_owned(),
        };
        let version_string = info.get_version_string();
        assert_eq!(version_string, "test".to_owned());
    }
}
