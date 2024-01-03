use std::path::{Path, PathBuf};

pub fn serialize_url(url: &str, file_ex: &str) -> String {
    let url = url.replace("blob:http://", "blob://");
    let url = url.replace("localhost:8080", "localhost");
    format!("{}{}", url, file_ex)
}

pub fn deserialize_url(url: &str) -> String {
    let url = url.replace("localhost", "localhost:8080");
    let ext = Path::new(&url).extension().unwrap().to_str().unwrap();
    let url = url.replace(&format!(".{}", ext), "");
    format!("blob:http://{}", url)
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
        let serialized = "blob://localhost/1234.png";

        assert_eq!(serialize_url(url, ".png"), serialized);

        let processed = serialized.replace("blob://", "");
        assert_eq!(deserialize_url(&processed), url);
    }
}
