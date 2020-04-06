use reqwest;
use reqwest::StatusCode;
use crate::client::EsClient;

#[derive(Debug)]
pub struct AliasResponse {
    aliases: Vec<AliasResults>,
}

#[derive(Debug)]
pub struct AliasResults {
    alias: String,
    index: String,
    filter: String,
    routing_index: String,
    routing_search: String,
}

pub async fn aliases_req(client: &EsClient) -> Result<AliasResponse, Box<dyn std::error::Error>> {
    let res = client.get(Some("_cat/aliases"))
        .send()
        .await?;

    let res = match res.status() {
        StatusCode::OK => {
            let text = res.text().await?;
            // Elasticsearch does not send back "json" formatted response, so the text ends up
            // being a str or all field values (if aliases) or empty string.
            if text.len() == 0 {
               AliasResponse { aliases: Vec::new() }
            } else {
                let aliases_text: Vec<&str> = text.split("\n").collect();
                let mut aliases = Vec::new();
                for alias in aliases_text {
                    if alias.len() > 0 {
                        let alias_vec: Vec<&str> = alias.split(" ").collect();
                        aliases.push(
                            AliasResults {
                                alias: alias_vec[0].to_owned(),
                                index: alias_vec[1].to_owned(),
                                filter: alias_vec[2].to_owned(),
                                routing_index: alias_vec[3].to_owned(),
                                routing_search: alias_vec[4].to_owned(),
                            });
                    }
                }
                AliasResponse { aliases }
            }
        },
        _ => panic!("Request failed in an unexpected way..."),
    };

    Ok(res)
}
