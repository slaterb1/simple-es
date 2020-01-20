use regex::Regex;
use reqwest;
use std::fmt;
use std::default::Default;
use tokio::runtime::Runtime;

use crate::info::es_info_req;

#[derive(Debug)]
pub enum Version {
    Es5,
    Es6,
    Es7,
}

/// EsClient used to make requests with Elasticsearch.
#[derive(Debug)]
pub struct EsClient {
    host: String,
    port: String,
    client: reqwest::Client,
    version: Version,
}

impl fmt::Display for EsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "host: {}, port: {}, client: {:?}, version: {:?}", self.host, self.port, self.client, self.version)
    }
}

impl Default for EsClient {
    fn default() -> EsClient {
        // Instantiate client.
        let mut client = EsClient {
            host: "http://localhost".to_owned(),
            port: "9200".to_owned(),
            client: reqwest::Client::new(),
            version: Version::Es6,
        };
        // Use client to get version and update version field.
        let version = client.get_version().unwrap();
        client.version = version;
        client
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
        // Instantiate client.
        let mut client = EsClient {
            host: host.to_owned(),
            port: port.to_string(),
            client: reqwest::Client::new(),
            version: Version::Es6,
        };
        // Use client to get version and update version field
        let version = client.get_version().unwrap();
        client.version = version;
        client
    }

    /// Helper function that sets the ES client version using the info request.
    fn get_version(&self) -> Result<Version, Box<dyn std::error::Error>> {
        // Setup runtime and get ES info from info request.
        let mut rt = Runtime::new()?;
        let info_req = es_info_req(self);
        let info = rt.block_on(info_req)?;

        // Parse and capture the version of ES.
        let re = Regex::new(r"[(\d+)(\d+)(\d+)]")?;
        let version_string = info.get_version_string();
        let caps = re.captures(&version_string).unwrap();
        let major_version = caps.get(0).unwrap();

        let version = match major_version.as_str() {
            "5" => Version::Es5,
            "6" => Version::Es6,
            "7" => Version::Es7,
            _ => panic!("Elasticsearch version found not currently supported. Please open up a ticket.")
        };

        Ok(version)
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

    // TODO: Update tests to include version after thinking through how to interact with ES.
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
