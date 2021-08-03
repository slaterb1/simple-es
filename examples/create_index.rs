use tokio::runtime::Runtime;

use simple_es::client::EsClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let rt = Runtime::new()?;
    let client = EsClient::default();

    // Create index.
    let index = rt.block_on(client.create_index("test"))?;
    println!("{:?}", index);

    Ok(())
}


