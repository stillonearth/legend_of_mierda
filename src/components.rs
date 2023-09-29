use rand;
use std::time::Duration;

use bevy::prelude::*;

use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::sprites::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Wall" => ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(1.0),
                friction: Friction::new(0.5),
                density: ColliderMassProperties::Density(15.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player {
    pub health: u8,
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Mierda {
    pub move_direction: Vec2,
    pub health: u8,
    pub hit_at: Option<Timer>,
}

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub character_animation: CharacterAnimation,
    pub animation_timer: AnimationTimer,
    pub player: Player,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
}

#[derive(Component, Clone, Default)]
pub struct DirectionUpdateTime {
    /// track when the bomb should explode (non-repeating timer)
    pub timer: Timer,
}

#[derive(Clone, Default, Bundle)]
pub struct MierdaBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub mierda: Mierda,
    pub collider_bundle: ColliderBundle,
    pub direction_update_time: DirectionUpdateTime,
}

pub fn create_mierda_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlas>,
) -> MierdaBundle {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;

    let collider_bundle = ColliderBundle {
        collider: Collider::cuboid(8., 8.),
        rigid_body: RigidBody::Dynamic,
        friction: Friction {
            coefficient: 20.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        rotation_constraints,
        ..Default::default()
    };

    let atlas_handle = load_texture_atlas(
        MIERDA_ASSET_SHEET,
        asset_server,
        5,
        1,
        None,
        16.,
        texture_atlasses,
    );

    let sprite_bundle = SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(4),
        ..default()
    };

    let mierda = Mierda {
        health: 100,
        move_direction: Vec2 {
            x: rand::random::<f32>() * 2.0 - 1.0,
            y: rand::random::<f32>() * 2.0 - 1.0,
        }
        .normalize(),
        hit_at: None,
    };

    MierdaBundle {
        sprite_bundle,
        collider_bundle,
        mierda,
        direction_update_time: DirectionUpdateTime {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
        },
        ..default()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
    sensor: Sensor,
}

impl FromWorld for PlayerSpritesheets {
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

        PlayerSpritesheets {
            player_atlas_1,
            player_atlas_2,
            mierda_atlas,
        }
    }
}
