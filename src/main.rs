#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_nine_slice_ui::*;
use bevy_rapier2d::prelude::*;
use components::Mierda;
use pecs::prelude::*;

mod ai;
mod components;
mod controls;
mod events;
mod ldtk;
mod physics;
mod sprites;
mod ui;
mod utils;

fn main() {
    let mut app = App::new();

    {
        let registry = app.world.resource_mut::<AppTypeRegistry>();
        let mut wr = registry.write();
        wr.register::<Mierda>();
    }

    app.add_plugins(DefaultPlugins)
        .add_plugins(PecsPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        // UI
        .add_systems(Startup, ui::draw_ui)
        // LDTK
        .add_plugins(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            // level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
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
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::MierdaBundle>("Mierda")
        // Enemy AI
        .add_systems(
            Update,
            (ai::mierda_activity, ai::update_mierdas_move_direction),
        )
        // Housekeeping
        .add_systems(
            Update,
            (
                ldtk::hide_dummy_mierdas,
                components::fix_missing_mierda_compontents,
            ),
        )
        // Sprites
        .init_resource::<sprites::PlayerSpritesheets>()
        .add_systems(Update, (sprites::animate_sprite, sprites::flash_sprite))
        // Controls
        .add_systems(Update, controls::controls)
        // Physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        // Events
        .add_systems(
            Update,
            (
                events::event_player_attack,
                events::event_player_hit,
                events::event_game_over,
                events::event_mierda_hit,
                events::event_spawn_mierda,
            ),
        )
        // Events: Collisions
        .add_systems(
            Update,
            (
                physics::handle_mierda_wall_collisions,
                physics::handle_player_mierda_collisions,
            ),
        )
        // App Events
        .add_event::<events::PlayerAttackEvent>()
        .add_event::<events::PlayerHitEvent>()
        .add_event::<events::GameOverEvent>()
        .add_event::<events::MierdaHitEvent>()
        .add_event::<events::SpawnMierdaEvent>();

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/example.ldtk"),
        ..Default::default()
    });
}
