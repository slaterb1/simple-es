use serde::Deserialize;

pub fn serialize_response<T>(raw_str: &str) -> serde_json::Result<T> 
    where for<'de> T: Deserialize<'de>
{
    let info: T = serde_json::from_str(raw_str)?;
    Ok(info)
}


