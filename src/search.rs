use reqwest;
use reqwest::StatusCode;
use serde_json::Value;
use serde::Deserialize;

use crate::client::EsClient;
use crate::utils::serialize_response;
use crate::errors::ESClientSearchFail;

#[derive(Deserialize, Debug, PartialEq)]
pub struct EsSearchResponse<T> {
    took: u16,
    timed_out: bool,
    #[serde(rename = "_shards")]
    shards: ShardResults,
    hits: HitResults<T>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ShardResults {
    total: u16,
    successful: u16,
    skipped: u16,
    failed: u16,
}

#[derive(Deserialize, Debug, PartialEq)]
struct HitResults<T> {
    hits: Vec<T>,
    total: u16,
    max_score: Option<f32>,
}



pub async fn search_req<T>(client: &EsClient, index: &str, doc_type: Option<&str>, query: Value) -> Result<EsSearchResponse<T>, Box<dyn std::error::Error>>
    where for<'de> T: Deserialize<'de>
{
    let res = client.post(index, doc_type, Some("_search"))
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

#[cfg(test)]
mod tests {
    use super::{
        search_req,
        EsSearchResponse,
        ShardResults,
        HitResults,
    };
    use crate::client::EsClient;

    use mockito::mock;
    use tokio::runtime::Runtime;
    use serde_json::json;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Results {
        a: String,
        b: u16,
    }

    #[test]
    fn successful_search_es6_with_results() {
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

        let _search_mock = mock("POST", "/test/_search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "took": 1,
                "timed_out": false,
                "_shards": {
                    "total": 1,
                    "successful": 1,
                    "skipped": 1,
                    "failed": 1
                },
                "hits": {
                    "hits": [
                        {
                            "a": "test",
                            "b": 1
                        }
                    ],
                    "total": 1,
                    "max_score": 1.0
                }
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = search_req::<Results>(
            &client,
            "test",
            None,
            json!({
                "query": {
                    "match_all": {}
                }
            })
        );
        
        let res = rt.block_on(res);
        let expected_res = EsSearchResponse {
                took: 1,
                timed_out: false,
                shards: ShardResults {
                    total: 1,
                    successful: 1,
                    skipped: 1,
                    failed: 1,
                },
                hits: HitResults {
                    hits: vec![
                        Results {
                            a: "test".to_owned(),
                            b: 1
                        },
                    ],
                    total: 1,
                    max_score: Some(1.0),
                },
        };
        assert_eq!(res.unwrap(), expected_res);
    }

    #[test]
    fn successful_search_es6_no_results() {
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

        let _search_mock = mock("POST", "/test/_search")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "took": 1,
                "timed_out": false,
                "_shards": {
                    "total": 1,
                    "successful": 1,
                    "skipped": 1,
                    "failed": 1
                },
                "hits": {
                    "hits": [],
                    "total": 0,
                    "max_score": null 
                }
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = search_req::<Results>(
            &client,
            "test",
            None,
            json!({
                "query": {
                    "match_all": {}
                }
            })
        );
        
        let res = rt.block_on(res);
        let expected_res = EsSearchResponse {
                took: 1,
                timed_out: false,
                shards: ShardResults {
                    total: 1,
                    successful: 1,
                    skipped: 1,
                    failed: 1,
                },
                hits: HitResults {
                    hits: vec![],
                    total: 0,
                    max_score: None,
                },
        };
        assert_eq!(res.unwrap(), expected_res);
    }

    #[test]
    fn failed_search_es6() {
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

        let _search_mock = mock("POST", "/test/_search")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "error": {
                    "root_case": [{
                        "error_type": "incorrect_query",
                        "reason": "could not parse query"
                },
                "status": 400
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = search_req::<Results>(
            &client,
            "test",
            None,
            json!({
                "query": {
                    "match_all": {}
                }
            })
        );
        
        let res = rt.block_on(res);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    #[should_panic]
    fn unexpected_error_es6() {
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

        let _index_mock = mock("POST", "/test/_search")
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "status": 500,
                "error": {
                    "root_cause": [{
                        "type": "index_create_failure",
                        "reason": "internal server error"
                    }]
                }
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = search_req::<Results>(
            &client,
            "test",
            None,
            json!({
                "query": {
                    "match_all": {}
                }
            })
        );
        let _ = rt.block_on(res);
    }
}
