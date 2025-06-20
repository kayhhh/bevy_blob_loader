use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_blob_loader::{BlobLoaderPlugin, path::serialize_url};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/blob.js")]
extern "C" {
    async fn get_blob() -> JsValue;
}

#[wasm_bindgen(start)]
async fn start() {
    // Call a JavsScript function to generate a blob URL
    let blob_url = get_blob().await.as_string().expect("blob url not a string");

    App::new()
        .add_plugins((
            // Must be added before AssetPlugin.
            BlobLoaderPlugin,
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
        ))
        .insert_resource(BlobToLoad(blob_url))
        .add_systems(Startup, load_blob_asset)
        .run();
}

#[derive(Resource)]
struct BlobToLoad(String);

fn load_blob_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    to_load: Res<BlobToLoad>,
) {
    // Serialize the blob URL into a format that Bevy can load.
    // This requires a file extension, which Bevy uses to determine which asset loader to use.
    let url = serialize_url(&to_load.0, "png");

    info!("Loading blob asset: {:?}", url);
    let handle: Handle<Image> = asset_server.load(url);

    // Use the loaded image as a texture
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: handle,
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    });
}
