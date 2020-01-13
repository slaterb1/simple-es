use tokio::runtime::Runtime;
use serde::Deserialize;
use serde_json::json;

use simple_es::search::search_req;
use simple_es::client::EsClient;

#[derive(Deserialize, Debug)]
struct Results {
    a: String,
    b: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let mut rt = Runtime::new().unwrap();
    let client = EsClient::default();

    // Print info on cluster.
    let search_future = search_req::<Results>(
        &client,
        "test",
        None,
        json!({
            "query": {
                "match_all": {}
            }
        })
    );

    let res = rt.block_on(search_future)?;
    println!("{:?}", res);

    Ok(())
}


