#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::log::LogPlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PresentMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::*;
use bevy_scene_hook::HookPlugin;
use ldtk::LEVEL_1_IID;
use pecs::prelude::*;

use cutscene::*;
use loading::*;
use menu::*;

mod audio;
mod controls;
mod cutscene;
mod entities;
mod gameover;
mod gameplay;
mod ldtk;
mod loading;
mod menu;
mod particles;
mod physics;
mod splashscreen;
mod sprites;
mod ui;
mod utils;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Splash,
    Menu,
    Cutscene,
    GamePlay,
    GameOver,
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
                        resolution: (900., 900.).into(),
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
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,legend_of_mierda=debug,bevy_animation=error,bevy_gltf=error".into(),
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
        .insert_resource(LevelSelection::iid(LEVEL_1_IID))
        .register_ldtk_int_cell::<ldtk::WallBundle>(1);

    app.run();
}

// -----------
// Game Plugin
// -----------

pub struct LegendOfMierdaPlugin;

impl Plugin for LegendOfMierdaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            entities::EntitiesPlugin,
            gameplay::GameplayPlugin,
            gameover::GameOverPlugin,
            splashscreen::SplashscreenPlugin,
        ))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_systems(
            OnEnter(GameState::GamePlay),
            (ldtk::spawn_game_world, ui::draw_ui),
        )
        .add_systems(
            OnExit(GameState::GamePlay),
            (ldtk::despawn_game_world, ui::despawn_ui),
        )
        .add_systems(
            Update,
            (
                ldtk::spawn_wall_collision,
                ldtk::camera_fit_inside_current_level,
                ldtk::update_level_selection,
            )
                .run_if(in_state(GameState::GamePlay)),
        ) // Housekeeping
        .add_systems(
            Update,
            (ldtk::hide_dummy_entities, ldtk::fix_missing_ldtk_entities)
                .run_if(in_state(GameState::GamePlay)),
        )
        // Sprites
        .add_systems(
            Update,
            (sprites::animate_player_sprite, sprites::flash_sprite)
                .run_if(in_state(GameState::GamePlay)),
        )
        // Controls
        .add_systems(
            Update,
            (controls::controls).run_if(in_state(GameState::GamePlay)),
        )
        // Particles
        .add_systems(
            Update,
            (particles::fix_particle_transform_z).run_if(in_state(GameState::GamePlay)),
        )
        // App Events
        .add_event::<ldtk::LevelChangeEvent>();
    }
}
