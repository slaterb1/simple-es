use tokio::runtime::Runtime;

use simple_es::client::EsClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let rt = Runtime::new()?;
    let client = EsClient::default();

    // Print info on cluster.
    let info = rt.block_on(client.info())?;
    println!("{:?}", info);

    Ok(())
}

