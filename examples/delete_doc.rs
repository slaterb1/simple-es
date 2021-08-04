use tokio::runtime::Runtime;
use serde::Serialize;

use simple_es::client::{EsClient, IndexPattern};
use simple_es::doc::{DocId, delete_doc_req};

#[derive(Serialize, Debug, Clone)]
struct Data {
    a: String,
    b: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup client and runtime.
    let rt = Runtime::new()?;
    let client = EsClient::default();

    let doc = Data {
        a: "test".to_owned(),
        b: 5
    };

    // Index doc into cluster with id.
    let index_doc_id_future = client.create_doc::<Data>(
        IndexPattern::Index("test"),
        DocId::Assigned("1"),
        None,
        doc.clone(),
    );
    
    // Delete doc just indexed.
    let delete_doc_future = delete_doc_req(
        &client,
        IndexPattern::Index("test"),
        DocId::Assigned("1")
    );

    // Call delete on doc that does not exist.
    let delete_unknown_doc = delete_doc_req(
        &client,
        IndexPattern::Index("test"),
        DocId::Assigned("1jfjf")
    );


    let res1 = rt.block_on(index_doc_id_future)?;
    let res2 = rt.block_on(delete_doc_future)?;
    let res4 = rt.block_on(delete_unknown_doc)?;
    println!("{:?}", res1);
    println!("{:?}", res2);
    println!("{:?}", res4);

    Ok(())
}



