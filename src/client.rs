use reqwest;
use std::fmt;
use std::default::Default;

/// EsClient used to make requests with Elasticsearch.
#[derive(Debug)]
pub struct EsClient {
    host: String,
    port: String,
    client: reqwest::Client,
}

impl fmt::Display for EsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "host: {}, port: {}, client: {:?}", self.host, self.port, self.client)
    }
}

impl Default for EsClient {
    fn default() -> EsClient {
        EsClient {
            host: "http://localhost".to_owned(),
            port: "9200".to_owned(),
            client: reqwest::Client::new(),
        }
    }
}

impl EsClient {
    /// Create new EsClient.
    ///
    /// # Arguments
    ///
    /// * `host` - Http host for Elasticsearch.
    /// * `port` - Port allocated for Elasticsearch connection.
    pub fn new(host: &str, port: u16) -> EsClient {
        EsClient {
            host: host.to_owned(),
            port: port.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Helper function to return url used in connection.
    pub fn get_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Convenient get wrapper for access to the client.
    pub fn get(&self) -> reqwest::RequestBuilder {
        let url = self.get_url();
        self.client.get(&url)
    }

    /// Convenient post wrapper for access to the client.
    pub fn post(&self, index: Option<&str>, doc_type: Option<&str>, action: Option<&str>) -> reqwest::RequestBuilder {
        let mut url = self.get_url();

        if let Some(index) = index {
            url = format!("{}/{}", url, index)
        }

        if let Some(doc_type) = doc_type  {
            url = format!("{}/{}", url, doc_type)
        }

        if let Some(action) = action  {
            url = format!("{}/{}", url, action)
        }
        self.client.post(&url)
    }
    /// Convenient put wrapper for access to the client.
    pub fn put(&self, index: Option<&str>, doc_type: Option<&str>) -> reqwest::RequestBuilder {
        let mut url = self.get_url();

        if let Some(index) = index {
            url = format!("{}/{}", url, index)
        }

        if let Some(doc_type) = doc_type  {
            url = format!("{}/{}", url, doc_type)
        }
        self.client.put(&url)
    }
}

#[cfg(test)]
mod tests {
    use super::EsClient;

    #[test]
    fn create_esclient() {
        let client = EsClient::new("http://localhost", 9200);
        assert_eq!(client.host, "http://localhost");
        assert_eq!(client.port, "9200");
    }

    #[test]
    fn create_default_esclient() {
        let client = EsClient::default();
        assert_eq!(client.host, "http://localhost");
        assert_eq!(client.port, "9200");
    }
}
