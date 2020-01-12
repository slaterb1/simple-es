use reqwest;

pub async fn search_req(client: &reqwest::Client) -> reqwest::Result<String> {
    let res = client.post("http://localhost:9200")
        .send()
        .await?;

    println!("{}", res.status());
    Ok("test".to_owned())
}

