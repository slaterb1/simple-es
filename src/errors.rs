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

/// ES Generic Failures
///
/// includes: search, and document creation/update
#[derive(Deserialize, Debug)]
pub struct ESGenericFail {
    error: ESGenericError,
    status: u16,
}

impl fmt::Display for ESGenericFail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {:?}, status: {}", self.error, self.status)
    }
}

impl Error for ESGenericFail {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Deserialize, Debug)]
struct ESGenericError {
    root_cause: Vec<ESGenericFailMetadata>,
}

#[derive(Deserialize, Debug)]
struct ESGenericFailMetadata {
    #[serde(rename = "type")]
    error_type: String,
    reason: String,
}
