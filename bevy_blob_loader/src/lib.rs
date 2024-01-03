use bevy::prelude::*;

pub mod path;
#[cfg(target_family = "wasm")]
pub mod source;

pub struct BlobLoaderPlugin;

impl Plugin for BlobLoaderPlugin {
    #[allow(unused)]
    fn build(&self, app: &mut App) {
        #[cfg(target_family = "wasm")]
        {
            use bevy::asset::io::{AssetSource, AssetSourceId};
            app.register_asset_source(
                AssetSourceId::Name("blob".into()),
                AssetSource::build().with_reader(|| Box::new(source::BlobAssetReader)),
            );
        }
    }
}
