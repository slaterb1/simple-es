use reqwest;
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

async fn es_info_req(client: reqwest::Client) -> reqwest::Result<String> {
    let res = client.get("http://localhost:9200")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

async fn search_req(client: reqwest::Client) -> reqwest::Result<String> {
    let res = client.post("http://localhost:9200")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

fn serialize_response<T>(raw_str: &str) -> serde_json::Result<T> 
    where for<'de> T: Deserialize<'de>
{
    let info: T = serde_json::from_str(raw_str)?;
    Ok(info)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = Runtime::new().unwrap();
    let client = reqwest::Client::new();
    let future = es_info_req(client);
    let raw_string = rt.block_on(future)?;
    let info = serialize_response::<EsInfo>(&raw_string)?;
    println!("{:?}", info);
    Ok(())
}
