use reqwest;
use reqwest::StatusCode;
use tokio::runtime::Runtime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct EsInfo {
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

#[derive(Debug)]
enum EsIndexCreate {
    Success(EsIndexCreateSuccess),
    Fail(EsIndexCreateFail),
}

#[derive(Deserialize, Debug)]
struct EsIndexCreateSuccess {
    acknowledged: bool,
    shards_acknowledged: bool,
    index: String,
}

#[derive(Deserialize, Debug)]
struct EsIndexCreateFail {
    error: EsIndexError,
    status: u16,
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

async fn es_info_req(client: &reqwest::Client) -> reqwest::Result<EsInfo> {
    let res = client.get("http://localhost:9200")
        .send()
        .await?
        .json::<EsInfo>()
        .await?;
    Ok(res)
}

async fn search_req(client: &reqwest::Client) -> reqwest::Result<String> {
    let res = client.post("http://localhost:9200")
        .send()
        .await?;

    println!("{}", res.status());
    Ok("test".to_owned())
}


async fn create_index_req(client: &reqwest::Client, index: &str) -> Result<EsIndexCreate, Box<dyn std::error::Error>> {
    let res = client.put(&format!("{}/{}", "http://localhost:9200", index))
        .send()
        .await?;

    let res = match res.status() {
        StatusCode::OK => {
            let text = res.text().await?;
            let data = serialize_response::<EsIndexCreateSuccess>(&text)?;
            EsIndexCreate::Success(data)
        },
        StatusCode::BAD_REQUEST => { 
            let text = res.text().await?;
            let data = serialize_response::<EsIndexCreateFail>(&text)?;
            EsIndexCreate::Fail(data)
        },
        _ => panic!("Request failed in an unexpected way..."),
    };
    Ok(res)
}

fn serialize_response<T>(raw_str: &str) -> serde_json::Result<T> 
    where for<'de> T: Deserialize<'de>
{
    let info: T = serde_json::from_str(raw_str)?;
    Ok(info)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let mut rt = Runtime::new().unwrap();
    let client = reqwest::Client::new();

    // Print info on cluster.
    let info_future = es_info_req(&client);
    let info = rt.block_on(info_future)?;
    //let info = serialize_response::<EsInfo>(&raw_string)?;
    println!("{:?}", info);

    // Create index.
    let index_future = create_index_req(&client, "test3");
    let index = rt.block_on(index_future)?;
    println!("{:?}", index);

    Ok(())
}
