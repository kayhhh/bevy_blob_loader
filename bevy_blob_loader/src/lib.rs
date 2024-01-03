use bevy::{
    asset::io::{
        AssetReader, AssetReaderError, AssetSource, AssetSourceId, PathStream, Reader, VecReader,
    },
    prelude::*,
    utils::BoxedFuture,
};
use js_sys::{Uint8Array, JSON};
use std::path::{Path, PathBuf};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

pub struct BlobLoaderPlugin;

impl Plugin for BlobLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            AssetSourceId::Name("blob".into()),
            AssetSource::build().with_reader(|| Box::new(BlobAssetReader)),
        );
    }
}

struct BlobAssetReader;

fn js_value_to_err<'a>(context: &'a str) -> impl FnOnce(JsValue) -> std::io::Error + 'a {
    move |value| {
        let message = match JSON::stringify(&value) {
            Ok(js_str) => format!("Failed to {context}: {js_str}"),
            Err(_) => {
                format!("Failed to {context} and also failed to stringify the JSValue of the error")
            }
        };

        std::io::Error::new(std::io::ErrorKind::Other, message)
    }
}

impl BlobAssetReader {
    async fn fetch_bytes<'a>(&self, path: PathBuf) -> Result<Box<Reader<'a>>, AssetReaderError> {
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_str(path.to_str().unwrap()))
            .await
            .map_err(js_value_to_err("fetch path"))?;
        let resp = resp_value
            .dyn_into::<Response>()
            .map_err(js_value_to_err("convert fetch to Response"))?;
        match resp.status() {
            200 => {
                let data = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
                let bytes = Uint8Array::new(&data).to_vec();
                let reader: Box<Reader> = Box::new(VecReader::new(bytes));
                Ok(reader)
            }
            404 => Err(AssetReaderError::NotFound(path)),
            status => Err(AssetReaderError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Encountered unexpected HTTP status {status}"),
            ))),
        }
    }
}

impl AssetReader for BlobAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let path = deserialize_path(path);
            info!("BlobAssetReader reading {:?}", path);
            self.fetch_bytes(path).await
        })
    }
    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let path = deserialize_path(path);
            self.fetch_bytes(path).await
        })
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        Box::pin(async move {
            Err(AssetReaderError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "BlobAssetReader does not support reading directories",
            )))
        })
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
        Box::pin(async move {
            Err(AssetReaderError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "BlobAssetReader does not support reading directories",
            )))
        })
    }
}

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
