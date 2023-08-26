#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            // level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Startup, systems::draw_ui)
        .init_resource::<components::PlayerSpritesheets>()
        .insert_resource(LevelSelection::Index(0))
        .add_systems(Update, systems::spawn_wall_collision)
        .add_systems(Update, systems::camera_fit_inside_current_level)
        .add_systems(Update, systems::update_level_selection)
        .add_systems(Update, systems::animate_sprite)
        .add_systems(Update, systems::controls)
        .add_systems(Update, systems::handle_collisions)
        .add_systems(Update, systems::mierda_movement)
        .add_systems(Update, systems::update_mierdas_move_direction)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        // .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::MierdaBundle>("Mierda")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/example.ldtk"),
        ..Default::default()
    });
}
