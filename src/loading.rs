use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;


pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        info!("here!");

        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        );

        app.add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, AvatarAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, CutsceneAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, FontAssets>(GameState::Loading);

        app.init_resource::<MaterialAssets>();
        app.init_resource::<MeshAssets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // #[asset(path = "audio/neural.mp3")]
    // pub neural: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/PixeloidMono-d94EV.ttf")]
    pub pixeloid_mono: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AvatarAssets {
    #[asset(path = "avatars/alextime.png")]
    pub alextime: Handle<Image>,
    #[asset(path = "avatars/gennadiy.png")]
    pub gennadiy: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CutsceneAssets {
    #[asset(path = "cutscenes/phone-call-1.png")]
    pub phone_call_1: Handle<Image>,
}

#[derive(Resource)]
pub struct MeshAssets {}

impl FromWorld for MeshAssets {
    fn from_world(_world: &mut World) -> Self {
        Self {}
    }
}

#[derive(Resource)]
pub struct MaterialAssets {
    pub black: Handle<StandardMaterial>,
    pub white: Handle<StandardMaterial>,
    pub yellow: Handle<StandardMaterial>,
    pub blue: Handle<StandardMaterial>,
    pub red: Handle<StandardMaterial>,
    pub transparent_white: Handle<StandardMaterial>,
    pub transparent_black: Handle<StandardMaterial>,
}

impl FromWorld for MaterialAssets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials_asset = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        MaterialAssets {
            black: materials_asset.add(bevy::prelude::Color::rgb(0., 0.1, 0.1).into()),
            white: materials_asset.add(bevy::prelude::Color::rgb(1., 0.9, 0.9).into()),
            red: materials_asset.add(bevy::prelude::Color::rgba(1., 0.1, 0.1, 0.5).into()),
            yellow: materials_asset.add(bevy::prelude::Color::YELLOW.into()),
            blue: materials_asset.add(bevy::prelude::Color::BLUE.into()),
            transparent_white: materials_asset
                .add(bevy::prelude::Color::rgba(1., 0.9, 0.9, 0.5).into()),
            transparent_black: materials_asset
                .add(bevy::prelude::Color::rgba(0., 0.1, 0.1, 0.5).into()),
        }
    }
}
