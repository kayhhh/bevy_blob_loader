use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_blob_loader::{path::serialize_url, BlobLoaderPlugin};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/blob.js")]
extern "C" {
    async fn get_blob() -> JsValue;
}

#[wasm_bindgen(start)]
async fn start() {
    let blob_url = get_blob().await.as_string().expect("blob url not a string");

    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            // Must be added before AssetPlugin (which is inside DefaultPlugins)
            BlobLoaderPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
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
    // We have to add the file extension to the url
    let url = serialize_url(&to_load.0, ".png");

    info!("Loading blob asset: {:?}", url);
    let handle: Handle<Image> = asset_server.load(url);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: handle,
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    });
}
