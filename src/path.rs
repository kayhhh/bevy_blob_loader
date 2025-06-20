use std::path::{Path, PathBuf};

use base64::engine::{Engine, general_purpose::URL_SAFE_NO_PAD};

/// Serialize a blob URL into a format that Bevy can recognize.
/// This looks like `blob://<base64-encoded-url>.<file-extension>`.
pub fn serialize_url(url: &str, file_ex: &str) -> String {
    let encoded = URL_SAFE_NO_PAD.encode(url.as_bytes());
    format!("blob://{}.{}", encoded, file_ex)
}

/// Deserialize a serialized blob URL back into its original form.
pub fn deserialize_url(url: &str) -> String {
    let ext = Path::new(&url).extension().unwrap().to_str().unwrap();
    let url = url.replace(&format!(".{}", ext), "").replace("blob://", "");
    let decoded = URL_SAFE_NO_PAD
        .decode(url.as_bytes())
        .expect("Failed to decode URL");
    String::from_utf8(decoded).expect("Failed to convert URL to UTF-8")
}

pub fn deserialize_path(path: &Path) -> PathBuf {
    let path_str = path.to_str().unwrap();
    let path_str = deserialize_url(path_str);
    PathBuf::from(path_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_url() {
        let url = "blob:http://localhost:8080/1234";
        let serialized = serialize_url(url, "png");

        // Processing done by Bevy
        let processed = serialized.replace("blob://", "");

        assert_eq!(deserialize_url(&processed), url);
    }
}
