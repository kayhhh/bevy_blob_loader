use bevy::{
    asset::io::{AssetReader, AssetReaderError, AssetSource, AssetSourceId, PathStream, Reader},
    prelude::*,
    utils::BoxedFuture,
};
use std::path::Path;

pub struct BlobLoaderPlugin;

impl Plugin for BlobLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            AssetSourceId::Name("blob".into()),
            AssetSource::build().with_reader(|| {
                Box::new(CustomAssetReader(AssetSource::get_default_reader(
                    "assets".to_string(),
                )()))
            }),
        );
    }
}

struct CustomAssetReader(Box<dyn AssetReader>);

impl AssetReader for CustomAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        info!("Reading {:?}", path);
        self.0.read(path)
    }
    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        self.0.read_meta(path)
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        self.0.read_directory(path)
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<bool, AssetReaderError>> {
        self.0.is_directory(path)
    }
}
