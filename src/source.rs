use bevy::{
    asset::io::{AssetReaderError, ErasedAssetReader, PathStream, Reader, VecReader},
    tasks::BoxedFuture,
};
use js_sys::{JSON, Uint8Array};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

use crate::path::deserialize_path;

pub struct BlobAssetReader;

impl BlobAssetReader {
    async fn fetch_bytes<'a>(&self, path: PathBuf) -> Result<Vec<u8>, AssetReaderError> {
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
                Ok(bytes)
            }
            404 => Err(AssetReaderError::NotFound(path)),
            status => Err(AssetReaderError::Io(Arc::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Encountered unexpected HTTP status {status}"),
            )))),
        }
    }

    async fn fetch_reader<'a>(
        &self,
        path: PathBuf,
    ) -> Result<Box<dyn Reader + 'a>, AssetReaderError> {
        let bytes = self.fetch_bytes(path).await?;
        let reader = Box::new(VecReader::new(bytes));
        Ok(reader)
    }
}

impl ErasedAssetReader for BlobAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<dyn Reader + 'a>, AssetReaderError>> {
        Box::pin(async move {
            let path = deserialize_path(path);
            self.fetch_reader(path).await
        })
    }
    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<dyn Reader + 'a>, AssetReaderError>> {
        Box::pin(async move {
            let path = deserialize_path(path);
            self.fetch_reader(path).await
        })
    }
    fn read_meta_bytes<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Vec<u8>, AssetReaderError>> {
        Box::pin(async move {
            let path = deserialize_path(path);
            self.fetch_bytes(path).await
        })
    }

    fn read_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        Box::pin(async move {
            Err(AssetReaderError::Io(Arc::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "BlobAssetReader does not support reading directories",
            ))))
        })
    }

    fn is_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
        Box::pin(async move {
            Err(AssetReaderError::Io(Arc::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "BlobAssetReader does not support reading directories",
            ))))
        })
    }
}

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
