#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use belly::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use pecs::prelude::*;

mod ai;
mod components;
mod controls;
mod events;
mod ldtk;
mod physics;
mod sprites;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BellyPlugin)
        .add_plugins(PecsPlugin)
        .add_plugins(WorldInspectorPlugin::new())
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
        // Sprites
        .init_resource::<sprites::PlayerSpritesheets>()
        .add_systems(Update, sprites::animate_sprite)
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
            (events::event_player_attack, events::event_player_hit),
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
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/example.ldtk"),
        ..Default::default()
    });
}
