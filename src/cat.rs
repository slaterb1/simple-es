use reqwest;
use reqwest::StatusCode;
use crate::client::EsClient;

#[derive(Debug, PartialEq)]
pub struct AliasResponse {
    aliases: Vec<AliasResults>,
}

#[derive(Debug, PartialEq)]
pub struct AliasResults {
    alias: String,
    index: String,
    filter: String,
    routing_index: String,
    routing_search: String,
}

pub async fn aliases_req(client: &EsClient) -> Result<AliasResponse, Box<dyn std::error::Error>> {
    let res = client.get(Some("_cat/aliases"))
        .send()
        .await?;

    let res = match res.status() {
        StatusCode::OK => {
            let text = res.text().await?;
            // Elasticsearch does not send back "json" formatted response, so the text ends up
            // being a str or all field values (if aliases) or empty string.
            if text.len() == 0 {
               AliasResponse { aliases: Vec::new() }
            } else {
                let aliases_text: Vec<&str> = text.split("\n").collect();
                let mut aliases = Vec::new();
                for alias in aliases_text {
                    if alias.len() > 0 {
                        let alias_vec: Vec<&str> = alias.split(" ").collect();
                        aliases.push(
                            AliasResults {
                                alias: alias_vec[0].to_owned(),
                                index: alias_vec[1].to_owned(),
                                filter: alias_vec[2].to_owned(),
                                routing_index: alias_vec[3].to_owned(),
                                routing_search: alias_vec[4].to_owned(),
                            });
                    }
                }
                AliasResponse { aliases }
            }
        },
        _ => panic!("Request failed in an unexpected way..."),
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::{
        aliases_req,
        AliasResponse,
        AliasResults,
    };
    use crate::client::EsClient;

    use mockito::mock;
    use tokio::runtime::Runtime;
    use serde_json::json;
    use serde::Deserialize;

    #[test]
    fn successful_aliases_es6_no_results() {
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

        let _aliases_mock = mock("GET", "/_cat/aliases")
            .with_status(200)
            .with_body("")
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = aliases_req(&client);
        let res = rt.block_on(res);

        let expected_res = AliasResponse {
            aliases: Vec::new()
        };
        assert_eq!(res.unwrap(), expected_res);
    }

    #[test]
    fn successful_aliases_es6_with_results() {
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

        // Formatting is based on how reqwest resolves the non json body ES sends back.
        let _aliases_mock = mock("GET", "/_cat/aliases")
            .with_status(200)
            .with_body("test_alias test - - -\n")
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = aliases_req(&client);
        let res = rt.block_on(res);

        let expected_res = AliasResponse {
            aliases: vec![
                AliasResults {
                    alias: "test_alias".to_owned(),
                    index: "test".to_owned(),
                    filter: "-".to_owned(),
                    routing_index: "-".to_owned(),
                    routing_search: "-".to_owned(),
                }
            ]
        };
        assert_eq!(res.unwrap(), expected_res);
    }
}
