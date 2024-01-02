use bevy::prelude::*;
use bevy_blob_loader::BlobLoaderPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/blob.js")]
extern "C" {
    async fn get_blob() -> JsValue;
}

#[wasm_bindgen(start)]
async fn start() {
    let blob_url = get_blob().await.as_string().expect("blob url not a string");

    info!("Blob URL: {}", blob_url);

    App::new()
        .add_plugins((
            // Must be added before AssetPlugin (which is inside DefaultPlugins)
            BlobLoaderPlugin,
            DefaultPlugins,
        ))
        .insert_resource(BlobToLoad(blob_url))
        .add_systems(Startup, load_blob_asset)
        .run();
}

#[derive(Resource)]
struct BlobToLoad(String);

fn load_blob_asset(asset_server: Res<AssetServer>, to_load: Res<BlobToLoad>) {
    let handle: Handle<Image> = asset_server.load(&to_load.0);
    info!("Loaded blob asset: {:?}", handle);
}
