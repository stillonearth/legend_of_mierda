#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::log::LogPlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PresentMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::*;
use bevy_scene_hook::HookPlugin;
use pecs::prelude::*;

use cutscene::*;
use loading::*;
use menu::*;

mod audio;
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
        // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set())
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Legend of Mierda".into(),
                        resolution: (1024., 1024.).into(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,legend_of_mierda=debug,bevy_animation=error".into(),
                    level: bevy::log::Level::DEBUG,
                }),
            AudioPlugin))
        .add_plugins(HookPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins((MenuPlugin, CutscenePlugin, LegendOfMierdaPlugin))
        .add_plugins(audio::InternalAudioPlugin)
        .add_plugins(PecsPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(ParticleSystemPlugin)
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
        .insert_resource(LevelSelection::Uid(0))
        .register_ldtk_int_cell::<ldtk::WallBundle>(1);

    app.run();
}

// -----------
// Game Plugin
// -----------

pub struct LegendOfMierdaPlugin;

impl Plugin for LegendOfMierdaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((entities::EntitiesPlugin, gameplay::GameplayPlugin))
            .add_systems(
                OnEnter(GameState::Gameplay),
                (spawn_game_world, ui::draw_ui),
            )
            .add_systems(
                Update,
                (
                    ldtk::spawn_wall_collision,
                    ldtk::camera_fit_inside_current_level,
                    ldtk::update_level_selection,
                ),
            ) // Housekeeping
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
    }
}

fn spawn_game_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/example.ldtk"),
        ..Default::default()
    });
}
