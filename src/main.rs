#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::*;
use pecs::prelude::*;

use cutscene::*;
use loading::*;
use menu::*;

mod controls;
mod cutscene;
mod entities;
mod gameplay;
mod ldtk;
mod loading;
mod menu;
mod particles;
mod physics;
mod sprites;
mod ui;
mod utils;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Cutscene,
    Gameplay,
}

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LoadingPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(CutscenePlugin)
        .add_plugins(PecsPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(ParticleSystemPlugin)
        // Game Plugins
        .add_plugins((entities::EntitiesPlugin, gameplay::GameplayPlugin))
        .add_systems(
            OnEnter(GameState::Gameplay),
            (spawn_game_world, ui::draw_ui),
        )
        // Physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        // LDTK
        .add_plugins(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(
            Update,
            (
                ldtk::spawn_wall_collision,
                ldtk::camera_fit_inside_current_level,
                ldtk::update_level_selection,
            ),
        )
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<ldtk::WallBundle>(1)
        // Housekeeping
        .add_systems(
            Update,
            (ldtk::hide_dummy_entities, ldtk::fix_missing_ldtk_entities),
        )
        // Sprites
        .add_systems(
            Update,
            (sprites::animate_player_sprite, sprites::flash_sprite),
        )
        // Controls
        .add_systems(Update, controls::controls)
        // Particles
        .add_systems(Update, particles::fix_particle_transform_z)
        // App Events
        .add_event::<ldtk::LevelChangeEvent>();

    app.run();
}

fn spawn_game_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/example.ldtk"),
        ..Default::default()
    });
}
