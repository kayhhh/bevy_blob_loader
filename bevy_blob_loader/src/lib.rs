//! This crate provides a [Bevy](https://bevyengine.org) plugin for loading assets from
//! JavaScript blob URLs. This is useful for taking input from the user within a browser,
//! such as a file upload or drag-and-drop.
//!
//! ## Usage
//!
//! After adding the plugin to your Bevy app, you can load assets from blob URLs like so:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_blob_loader::path::serialize_url;
//!
//! fn my_system(asset_server: Res<AssetServer>) {
//!   let blob_url = "blob:http://example.com/1234-5678-9012-3456";
//!
//!   // Note, we have to serialize the URL into a special format for Bevy to recognize it.
//!   // This takes in the file extension of the asset, which Bevy uses to determine how to
//!   // process the fetched asset.
//!   let serialized = serialize_url(&blob_url, ".png");
//!
//!   let handle: Handle<Image> = asset_server.load(serialized);
//! }
//! ```
//!

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
