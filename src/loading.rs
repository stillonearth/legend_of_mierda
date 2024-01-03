use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{sprites::*, GameState};

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
        app.init_resource::<CharacterSpritesheets>();
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
    #[asset(path = "cutscenes/main-menu.png")]
    pub main_menu: Handle<Image>,
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

#[derive(Resource)]
pub struct CharacterSpritesheets {
    pub player_atlas_1: Handle<TextureAtlas>,
    pub player_atlas_2: Handle<TextureAtlas>,
    pub mierda_atlas: Handle<TextureAtlas>,
    pub pendejo_atlas_1: Handle<TextureAtlas>,
    pub pendejo_atlas_2: Handle<TextureAtlas>,
}

impl FromWorld for CharacterSpritesheets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let asset_server_world_borrow = world.get_resource::<AssetServer>();
        let asset_server = asset_server_world_borrow.as_deref().unwrap();

        let mut texture_atlasses_world_borrow = world.get_resource_mut::<Assets<TextureAtlas>>();
        let texture_atlasses = texture_atlasses_world_borrow.as_deref_mut().unwrap();

        let player_atlas_1 = load_texture_atlas(
            PLAYER_ASSET_SHEET_1,
            asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            None,
            64.,
            texture_atlasses,
        );

        let pendejo_atlas_1 = load_texture_atlas(
            PENDEJO_SPRITE_SHEETS[0].0,
            asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            None,
            64.,
            texture_atlasses,
        );

        let pendejo_atlas_2 = load_texture_atlas(
            PENDEJO_SPRITE_SHEETS[1].0,
            asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            None,
            64.,
            texture_atlasses,
        );

        let player_atlas_2 = load_texture_atlas(
            PLAYER_ASSET_SHEET_2,
            asset_server,
            SHEET_2_COLUMNS,
            SHEET_2_ROWS,
            None,
            64. * 3.,
            texture_atlasses,
        );

        let mierda_atlas = load_texture_atlas(
            MIERDA_ASSET_SHEET,
            asset_server,
            5,
            1,
            None,
            16.0,
            texture_atlasses,
        );

        CharacterSpritesheets {
            player_atlas_1,
            player_atlas_2,
            mierda_atlas,
            pendejo_atlas_1,
            pendejo_atlas_2,
        }
    }
}

pub fn load_texture_atlas(
    path: &str,
    asset_server: &AssetServer,
    sheet_columns: usize,
    sheet_rows: usize,
    padding: Option<Vec2>,
    sprite_size: f32,
    texture_atlasses: &mut Assets<TextureAtlas>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(path);

    let atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::ONE * sprite_size,
        sheet_columns,
        sheet_rows,
        padding,
        None,
    );

    texture_atlasses.add(atlas)
}
