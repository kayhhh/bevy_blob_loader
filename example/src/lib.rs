use bevy::prelude::*;
use bevy_blob_loader::BlobLoaderPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/blob.js")]
extern "C" {
    fn get_blob() -> String;
}

#[wasm_bindgen(start)]
fn start() {
    App::new()
        .add_plugins((
            // Must be added before AssetPlugin (which is inside DefaultPlugins)
            BlobLoaderPlugin,
            DefaultPlugins,
        ))
        .add_systems(Startup, load_blob_asset)
        .run();
}

fn load_blob_asset(asset_server: Res<AssetServer>) {
    let url = get_blob();
    info!("Blob: {}", url);

    let _blob_handle: Handle<Image> = asset_server.load(url);
}
