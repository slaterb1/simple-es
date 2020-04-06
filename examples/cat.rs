use tokio::runtime::Runtime;

use simple_es::cat::aliases_req;
use simple_es::client::EsClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let mut rt = Runtime::new().unwrap();
    let client = EsClient::default();

    // Print info on aliases.
    let aliases_future = aliases_req(&client);
    let info = rt.block_on(aliases_future)?;
    println!("{:?}", info);

    Ok(())
}


