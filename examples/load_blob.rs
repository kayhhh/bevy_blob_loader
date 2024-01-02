use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/examples/load_blob.js")]
extern "C" {
    fn get_blob() -> String;
}

fn main() {
    App::new()
        .add_plugins((
            // Must be added before AssetPlugin (which is inside DefaultPlugins)
            bevy_blob_loader::BlobLoaderPlugin,
            DefaultPlugins,
        ))
        .add_systems(Startup, load_blob_asset)
        .run();
}

fn load_blob_asset(asset_server: Res<AssetServer>) {
    let url = get_blob();
    info!("Blob: {}", url);

    let blob_handle: Handle<Image> = asset_server.load(url);
}
