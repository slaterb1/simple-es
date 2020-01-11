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

async fn call_es(client: reqwest::Client) -> reqwest::Result<()> {
    let res = client.get("http://localhost:9200")
        .send()
        .await?
        .text()
        .await?;

    let data: EsInfo = serde_json::from_str(&res).unwrap();
    println!("{:?}", data);
    Ok(())
}

fn main() -> reqwest::Result<()> {
    let mut rt = Runtime::new().unwrap();
    let client = reqwest::Client::new();
    let future = call_es(client);
    rt.block_on(future)?;
    Ok(())
}
