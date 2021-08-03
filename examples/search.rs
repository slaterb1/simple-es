use tokio::runtime::Runtime;
use serde::Deserialize;
use serde_json::json;

use simple_es::client::EsClient;

#[derive(Deserialize, Debug)]
struct Results {
    a: String,
    b: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let rt = Runtime::new()?;
    let client = EsClient::default();

    // Return search of all documents in index "test".
    let search_future = client.search::<Results>(
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


