use serde::Deserialize;
use std::error::Error;
use std::fmt;

/// Index creation failures
///
#[derive(Deserialize, Debug)]
pub struct ESClientCreateIndexFail {
    error: ESClientCreateIndexError,
    status: u16,
}

impl fmt::Display for ESClientCreateIndexFail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {:?}, status: {}", self.error, self.status)
    }
}

impl Error for ESClientCreateIndexFail {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Deserialize, Debug)]
struct ESClientCreateIndexError {
    root_cause: Vec<ESClientCreateIndexFailMetadata>,
}

#[derive(Deserialize, Debug)]
struct ESClientCreateIndexFailMetadata {
    #[serde(rename = "type")]
    error_type: String,
    reason: String,
    index_uuid: String,
    index: String,
}

/// Document search failtures
///
#[derive(Deserialize, Debug)]
pub struct ESClientSearchFail {
    error: ESClientSearchError,
    status: u16,
}

impl fmt::Display for ESClientSearchFail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {:?}, status: {}", self.error, self.status)
    }
}

impl Error for ESClientSearchFail {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Deserialize, Debug)]
struct ESClientSearchError {
    root_cause: Vec<ESClientSearchFailMetadata>,
}

#[derive(Deserialize, Debug)]
struct ESClientSearchFailMetadata {
    #[serde(rename = "type")]
    error_type: String,
    reason: String,
}
