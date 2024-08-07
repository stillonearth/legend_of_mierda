use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{sprites::*, GameState};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Splash),
        );

        app.add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, AvatarAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, CutsceneAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, SceneAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, AnimationAssets>(GameState::Loading);
        app.add_collection_to_loading_state::<_, StaticSpriteAssets>(GameState::Loading);

        app.init_resource::<FontAssets>();
        app.init_resource::<MaterialAssets>();
        app.init_resource::<MeshAssets>();
        app.init_resource::<CharacterSpritesheets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/biboran.ogg")]
    pub biboran: Handle<AudioSource>,
    #[asset(path = "audio/mierda.ogg")]
    pub mierda: Handle<AudioSource>,
    #[asset(path = "audio/slash.ogg")]
    pub slash: Handle<AudioSource>,
    #[asset(path = "audio/hit.ogg")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "audio/hurt.ogg")]
    pub hurt: Handle<AudioSource>,
    #[asset(path = "audio/gameover.ogg")]
    pub gameover: Handle<AudioSource>,
    #[asset(path = "audio/mexico.ogg")]
    pub mexico: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "models/biboran.glb#Scene0")]
    pub biboran: Handle<Scene>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct AnimationAssets {
    #[asset(path = "models/biboran.glb#Animation0")]
    pub biboran: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    pub pixeloid_mono: Handle<Font>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();

        FontAssets {
            pixeloid_mono: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct StaticSpriteAssets {
    #[asset(path = "sprites/arrow.png")]
    pub arrow: Handle<Image>,
    #[asset(path = "sprites/speargun-wide.png")]
    pub speargun: Handle<Image>,
    #[asset(path = "sprites/speargun-arrow.png")]
    pub speargun_arrow: Handle<Image>,
    #[asset(path = "sprites/pill.png")]
    pub pill: Handle<Image>,
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
    #[asset(path = "cutscenes/splash.png")]
    pub splash: Handle<Image>,
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
    // pub psychiatrist_atlas: Handle<TextureAtlas>,
}

impl FromWorld for CharacterSpritesheets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let asset_server_world_borrow = world.get_resource::<AssetServer>();
        let asset_server = asset_server_world_borrow.as_deref().unwrap();

        let mut texture_atlasses_world_borrow = world.get_resource_mut::<Assets<TextureAtlas>>();
        let texture_atlasses = texture_atlasses_world_borrow.as_deref_mut().unwrap();

        let player_atlas_1 = load_texture_atlas(
            PLAYER_ASSET_SHEET_1.to_string(),
            asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            None,
            Vec2::ONE * 64.,
            texture_atlasses,
        );

        let pendejo_atlas_1 = load_texture_atlas(
            PENDEJO_SPRITE_SHEETS[0].0.to_string(),
            asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            None,
            Vec2::ONE * 64.,
            texture_atlasses,
        );

        let pendejo_atlas_2 = load_texture_atlas(
            PENDEJO_SPRITE_SHEETS[1].0.to_string(),
            asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            None,
            Vec2::ONE * 64.,
            texture_atlasses,
        );

        let player_atlas_2 = load_texture_atlas(
            PLAYER_ASSET_SHEET_2.to_string(),
            asset_server,
            SHEET_2_COLUMNS,
            SHEET_2_ROWS,
            None,
            Vec2::ONE * 64. * 3.,
            texture_atlasses,
        );

        let mierda_atlas = load_texture_atlas(
            MIERDA_ASSET_SHEET.to_string(),
            asset_server,
            5,
            1,
            None,
            Vec2::ONE * 16.0,
            texture_atlasses,
        );

        // let psychiatrist_atlas = load_texture_atlas(
        //     PSYCHIATRIST_ASSET_SHEET.to_string(),
        //     asset_server,
        //     1,
        //     1,
        //     None,
        //     Vec2::new(32., 32.),
        //     texture_atlasses,
        // );

        CharacterSpritesheets {
            player_atlas_1,
            player_atlas_2,
            mierda_atlas,
            pendejo_atlas_1,
            pendejo_atlas_2,
            // psychiatrist_atlas,
        }
    }
}

pub fn load_texture_atlas(
    path: String,
    asset_server: &AssetServer,
    sheet_columns: usize,
    sheet_rows: usize,
    padding: Option<Vec2>,
    sprite_size: Vec2,
    texture_atlasses: &mut Assets<TextureAtlas>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(path);

    let atlas = TextureAtlas::from_grid(
        texture_handle,
        sprite_size,
        sheet_columns,
        sheet_rows,
        padding,
        None,
    );

    texture_atlasses.add(atlas)
}
