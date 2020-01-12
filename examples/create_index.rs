use tokio::runtime::Runtime;

use simple_es::index::create_index_req;
use simple_es::client::EsClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let mut rt = Runtime::new().unwrap();
    let client = EsClient::default();

    // Create index.
    let index_future = create_index_req(&client, "test4");
    let index = rt.block_on(index_future)?;
    println!("{:?}", index);

    Ok(())
}


