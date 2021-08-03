use tokio::runtime::Runtime;

use simple_es::info::es_info_req;
use simple_es::client::EsClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let rt = Runtime::new()?;
    let client = EsClient::default();

    // Print info on cluster.
    let info_future = es_info_req(&client);
    let info = rt.block_on(info_future)?;
    println!("{:?}", info);

    Ok(())
}

