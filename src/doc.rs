use reqwest;
use reqwest::StatusCode;

use serde::{ Deserialize, Serialize };
use crate::{
    client::{
        EsClient,
        IndexPattern,
    },
    utils::serialize_response,
    errors::{
        ESGenericFail,
        ESMissingId,
    }
};

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
pub struct EsDeleteDocResponse {
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

/// Assign an id at doc creation time or let system decide.
pub enum DocId<'a> {
    Unassigned,
    Assigned(&'a str),
}

pub async fn index_doc_req<'a, T: Serialize>(
    client: &EsClient,
    write_on: IndexPattern<'a>,
    id: DocId<'a>,
    operation: Option<&str>,
    data: T
    ) -> Result<EsIndexDocResponse, Box<dyn std::error::Error>> 
{
    let (index, doc_type) = match write_on {
        IndexPattern::Index(index) => (index, None),
        IndexPattern::IndexType(index, doc_type) => (index, Some(doc_type))
    };

    // Check if id is passed to use either PUT method or POST.
    let res = match id {
        DocId::Assigned(id) => client.put_doc(index, doc_type, id, operation)
            .json(&data)
            .send()
            .await?,
        DocId::Unassigned => client.post_doc(index, doc_type)
            .json(&data)
            .send()
            .await?,
    };

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
            let data = serialize_response::<ESGenericFail>(&text)?;
            return Err(Box::new(data));
        },
        _ => panic!("Request failed in an unexpected way..."),
    };
    Ok(res)
}

pub async fn delete_doc_req<'a>(
    client: &EsClient,
    delete_on: IndexPattern<'a>,
    id: DocId<'a>
) -> Result<EsDeleteDocResponse, Box<dyn std::error::Error>>
{
    let (index, doc_type) = match delete_on {
        IndexPattern::Index(index) => (index, None),
        IndexPattern::IndexType(index, doc_type) => (index, Some(doc_type))
    };

    let res = match id {
        DocId::Assigned(id) => client.delete_doc_by_id(index, doc_type, id)
            .send()
            .await?,
        DocId::Unassigned => return Err(Box::new(ESMissingId::new()))
    };

    let res = match res.status() {
        StatusCode::OK => {
            let text = res.text().await?;
            let data = serialize_response::<EsDeleteDocResponse>(&text)?;
            data
        },
        StatusCode::NOT_FOUND => {
            let text = res.text().await?;
            let data = serialize_response::<EsDeleteDocResponse>(&text)?;
            data
        },
        _ => panic!("Request failed in an unexpected way..."),
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::{
        index_doc_req,
        delete_doc_req,
        EsIndexDocResponse,
        EsDeleteDocResponse,
        ShardResults,
        DocId,
    };
    use crate::{
        client::{
            EsClient,
            IndexPattern,
        },
    };

    use mockito::mock;
    use tokio::runtime::Runtime;
    use serde::Serialize;

    #[derive(Serialize, Debug, PartialEq)]
    struct Data {
        a: String,
        b: u16,
    }

    #[test]
    fn successful_create_doc_with_id_es6() {
        let rt = Runtime::new().unwrap();
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

        let _create_doc_mock = mock("PUT", "/test/_doc/1")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "_index": "test",
                "_type": "_doc",
                "_id": "1",
                "_version": 1,
                "result": "created",
                "_shards": {
                    "total": 2,
                    "successful": 1,
                    "failed": 0
                },
                "_seq_no": 0,
                "_primary_term": 1
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let doc = Data {
            a: "test".to_owned(),
            b: 5,
        };
        let res = index_doc_req::<Data>(
            &client,
            IndexPattern::Index("test"),
            DocId::Assigned("1"),
            None,
            doc,
        );

        let res = rt.block_on(res);
        let expected_res = EsIndexDocResponse {
            index: "test".to_owned(),
            doc_type: "_doc".to_owned(),
            id: "1".to_owned(),
            version: 1,
            result: "created".to_owned(),
            shards: ShardResults {
                total: 2,
                successful: 1,
                failed: 0,
            },
            seq_no: 0,
            primary_term: 1,
        };
        assert_eq!(res.unwrap(), expected_res);
    }

    #[test]
    fn successful_create_doc_without_id_es6() {
        let rt = Runtime::new().unwrap();
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

        let _create_doc_mock = mock("POST", "/test/_doc")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "_index": "test",
                "_type": "_doc",
                "_id": "abcdefg",
                "_version": 1,
                "result": "created",
                "_shards": {
                    "total": 2,
                    "successful": 1,
                    "failed": 0
                },
                "_seq_no": 0,
                "_primary_term": 1
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let doc = Data {
            a: "test".to_owned(),
            b: 5,
        };
        let res = index_doc_req::<Data>(
            &client,
            IndexPattern::Index("test"),
            DocId::Unassigned,
            None,
            doc,
        );

        let res = rt.block_on(res);
        let expected_res = EsIndexDocResponse {
            index: "test".to_owned(),
            doc_type: "_doc".to_owned(),
            id: "abcdefg".to_owned(),
            version: 1,
            result: "created".to_owned(),
            shards: ShardResults {
                total: 2,
                successful: 1,
                failed: 0,
            },
            seq_no: 0,
            primary_term: 1,
        };
        assert_eq!(res.unwrap(), expected_res);
    }

    #[test]
    fn successful_update_doc_with_id_es6() {
        let rt = Runtime::new().unwrap();
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

        let _create_doc_mock = mock("PUT", "/test/_doc/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "_index": "test",
                "_type": "_doc",
                "_id": "1",
                "_version": 1,
                "result": "updated",
                "_shards": {
                    "total": 2,
                    "successful": 1,
                    "failed": 0
                },
                "_seq_no": 0,
                "_primary_term": 1
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let doc = Data {
            a: "test".to_owned(),
            b: 5,
        };
        let res = index_doc_req::<Data>(
            &client,
            IndexPattern::Index("test"),
            DocId::Assigned("1"),
            None,
            doc,
        );

        let res = rt.block_on(res);
        let expected_res = EsIndexDocResponse {
            index: "test".to_owned(),
            doc_type: "_doc".to_owned(),
            id: "1".to_owned(),
            version: 1,
            result: "updated".to_owned(),
            shards: ShardResults {
                total: 2,
                successful: 1,
                failed: 0,
            },
            seq_no: 0,
            primary_term: 1,
        };
        assert_eq!(res.unwrap(), expected_res);
    }

    #[test]
    fn successful_update_doc_without_id_es6() {
        let rt = Runtime::new().unwrap();
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

        let _create_doc_mock = mock("POST", "/test/_doc")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "_index": "test",
                "_type": "_doc",
                "_id": "abcdefg",
                "_version": 1,
                "result": "updated",
                "_shards": {
                    "total": 2,
                    "successful": 1,
                    "failed": 0
                },
                "_seq_no": 0,
                "_primary_term": 1
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let doc = Data {
            a: "test".to_owned(),
            b: 5,
        };
        let res = index_doc_req::<Data>(
            &client,
            IndexPattern::Index("test"),
            DocId::Unassigned,
            None,
            doc,
        );

        let res = rt.block_on(res);
        let expected_res = EsIndexDocResponse {
            index: "test".to_owned(),
            doc_type: "_doc".to_owned(),
            id: "abcdefg".to_owned(),
            version: 1,
            result: "updated".to_owned(),
            shards: ShardResults {
                total: 2,
                successful: 1,
                failed: 0,
            },
            seq_no: 0,
            primary_term: 1,
        };
        assert_eq!(res.unwrap(), expected_res);
    }

    #[test]
    fn failed_create_doc_with_id_es6() {
        let rt = Runtime::new().unwrap();
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

        let _create_doc_mock = mock("PUT", "/test/doc/1")
            .with_status(400)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "error": {
                    "root_cause": [{
                        "type": "illegal_argument_exception",
                        "reason": "Rejecting mapping update to [test] as the final mapping would have more than 1 type: [_doc, doc]"
                    }],
                    "type": "illegal_argument_exception",
                    "reason": "Rejecting mapping update to [test] as the final mapping would have more than 1 type: [_doc, doc]"
                },
                "status": 400 
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let doc = Data {
            a: "test".to_owned(),
            b: 5,
        };
        let res = index_doc_req::<Data>(
            &client,
            IndexPattern::IndexType("test", "doc"),
            DocId::Assigned("1"),
            None,
            doc,
        );

        let res = rt.block_on(res);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    #[should_panic]
    fn unexpected_error_es6() {
        let rt = Runtime::new().unwrap();
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

        let _create_doc_mock = mock("PUT", "/test/_doc/1")
            .with_status(501)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "error": {
                    "root_cause": [{
                        "type": "corrupt request query",
                        "reason": "index query is wrong"
                    }],
                    "type": "corrupt request query",
                    "reason": "index query is wrong"
                },
                "status": 501 
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let doc = Data {
            a: "test".to_owned(),
            b: 5,
        };
        let res = index_doc_req::<Data>(
            &client,
            IndexPattern::IndexType("test", "doc"),
            DocId::Assigned("1"),
            None,
            doc,
        );

        let _res = rt.block_on(res);
    }

    #[test]
    fn successful_delete_doc_with_id_es6() {
        let rt = Runtime::new().unwrap();
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

        let _delete_doc_mock = mock("DELETE", "/test/_doc/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "_index": "test",
                "_type": "_doc",
                "_id": "1",
                "_version": 1,
                "result": "deleted",
                "_shards": {
                    "total": 2,
                    "successful": 1,
                    "failed": 0
                },
                "_seq_no": 0,
                "_primary_term": 1
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = delete_doc_req(
            &client,
            IndexPattern::Index("test"),
            DocId::Assigned("1"),
        );

        let res = rt.block_on(res);
        let expected_res = EsDeleteDocResponse {
            index: "test".to_owned(),
            doc_type: "_doc".to_owned(),
            id: "1".to_owned(),
            version: 1,
            result: "deleted".to_owned(),
            shards: ShardResults {
                total: 2,
                successful: 1,
                failed: 0,
            },
            seq_no: 0,
            primary_term: 1,
        };
        assert_eq!(res.unwrap(), expected_res);
    }

    #[test]
    fn fail_delete_doc_with_missing_id_es6() {
        let rt = Runtime::new().unwrap();
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

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = delete_doc_req(
            &client,
            IndexPattern::Index("test"),
            DocId::Unassigned,
        );

        let res = rt.block_on(res);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn fail_delete_doc_with_unknown_id_es6() {
        let rt = Runtime::new().unwrap();
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

        let _delete_doc_mock = mock("DELETE", "/test/_doc/19393")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "_index": "test",
                "_type": "_doc",
                "_id": "19393",
                "_version": 1,
                "result": "not_found",
                "_shards": {
                    "total": 2,
                    "successful": 1,
                    "failed": 0
                },
                "_seq_no": 0,
                "_primary_term": 1
            }"#)
            .create();

        let client = EsClient::new("http://127.0.0.1", 1234);
        let res = delete_doc_req(
            &client,
            IndexPattern::Index("test"),
            DocId::Assigned("19393"),
        );

        let res = rt.block_on(res);
        let expected_res = EsDeleteDocResponse {
            index: "test".to_owned(),
            doc_type: "_doc".to_owned(),
            id: "19393".to_owned(),
            version: 1,
            result: "not_found".to_owned(),
            shards: ShardResults {
                total: 2,
                successful: 1,
                failed: 0,
            },
            seq_no: 0,
            primary_term: 1,
        };
        assert_eq!(res.unwrap(), expected_res);
    }
}
