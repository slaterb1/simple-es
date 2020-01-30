use tokio::runtime::Runtime;
use serde::Serialize;
use serde_json::json;

use simple_es::doc::index_doc_req;
use simple_es::client::EsClient;

#[derive(Serialize, Debug)]
struct Data {
    a: String,
    b: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let mut rt = Runtime::new().unwrap();
    let client = EsClient::default();

    let doc = Data {
        a: "test".to_owned(),
        b: 5
    };

    // Index doc into cluster.
    let index_doc_future = index_doc_req::<Data>(
        &client,
        "test",
        None,
        Some("1"),
        None,
        doc,
    );

    let res = rt.block_on(index_doc_future)?;
    println!("{:?}", res);

    Ok(())
}



