#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::log::LogPlugin;
use bevy::render::camera::RenderTarget;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PresentMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_magic_light_2d::gi::compositing::{setup_post_processing_camera, CameraTargets};
use bevy_magic_light_2d::gi::resource::{BevyMagicLight2DSettings, LightPassParams};
use bevy_magic_light_2d::gi::BevyMagicLight2DPlugin;
use bevy_magic_light_2d::{FloorCamera, SpriteCamera};
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::*;
use bevy_scene_hook::HookPlugin;
use bevy_tweening::TweeningPlugin;
use ldtk::LEVEL_1_IID;
use pecs::prelude::*;

use cutscene::*;
use loading::*;
use menu::*;
use postprocessing::{PostProcessSettings};

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
mod postprocessing;
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
            AudioPlugin, /*PostProcessPlugin*/))
        .add_plugins((HookPlugin, PecsPlugin, TweeningPlugin, BevyMagicLight2DPlugin))
        .add_plugins((LoadingPlugin, MenuPlugin, CutscenePlugin, LegendOfMierdaPlugin))
        .add_plugins(audio::InternalAudioPlugin)
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
        // Magic Light
        .insert_resource(BevyMagicLight2DSettings {
            light_pass_params: LightPassParams {
                reservoir_size: 8,
                smooth_kernel_size: (3, 3),
                direct_light_contrib: 0.5,
                indirect_light_contrib: 0.5,
                ..default()
            },
            ..default()
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

fn spawn_camera(mut commands: Commands, camera_targets: Res<CameraTargets>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                target: RenderTarget::Image(camera_targets.floor_target.clone()),
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new("main_camera"),
        FloorCamera,
        PostProcessSettings {
            width: 712.,
            height: 712.,
            ..default()
        },
        SpriteCamera,
    ));
}

pub struct LegendOfMierdaPlugin;

impl Plugin for LegendOfMierdaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            entities::EntitiesPlugin,
            gameplay::GameplayPlugin,
            gameover::GameOverPlugin,
            splashscreen::SplashscreenPlugin,
        ))
        .add_systems(Startup, (spawn_camera).after(setup_post_processing_camera))
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
