# simple-es
A simple Elasticsearch client for Rust.

[![MIT-LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](MIT-LICENSE)
[![Apache-LICENSE](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](Apache-LICENSE)

The objective of this project is to follow idiomatic Rust conventions and build an ES client that can connect with any version of ES that is 5.6+. All features of Elasticsearch will eventually be ported over. Features that have issues created are the current priority. Check there if you want to see the progression of the project.

This is an asynchronous client that is using standard async/.await and tokio runtime. See the examples for implementation details and how to pull the client into one of your projects.

This library will support "free" text json body requests to make requests to ES, it will not have a query builder, so request structure will not be checked at compile time. This is an intentional decision to give the library more flexibility and to make it easier to interface with for custom interactions with ES.

## Example
```rust
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
```
