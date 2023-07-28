use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

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

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    // pub animation_indices: AnimationIndices,
    pub character_animation: CharacterAnimation,
    pub animation_timer: AnimationTimer,
    // pub transform: Transform,
    pub player: Player,
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

const COLUMNS: usize = 13;
const ROWS: usize = 21;
const N_FRAMES_WALK: usize = 8;

#[allow(dead_code)]
#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Clone, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Default, Copy, PartialEq)]
pub enum AnimationDirection {
    Left,
    Right,
    Up,
    #[default]
    Down,
}

#[derive(Clone, Default, Copy, PartialEq)]
pub enum AnimationType {
    #[default]
    Walk,
    Stand,
    Attack,
}

pub fn get_animation_indices(
    animation_type: AnimationType,
    animation_direction: AnimationDirection,
) -> AnimationIndices {
    let mut first = 0;
    let mut last = 0;

    // Walk

    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Right {
        first = COLUMNS * 11 + 1;
        last = COLUMNS * 11 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Left {
        first = COLUMNS * 9 + 1;
        last = COLUMNS * 9 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Up {
        first = COLUMNS * 8 + 1;
        last = COLUMNS * 8 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Down {
        first = COLUMNS * 10 + 1;
        last = COLUMNS * 10 + N_FRAMES_WALK;
    }

    // Stand

    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Right {
        first = COLUMNS * 11;
        last = last;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Left {
        first = COLUMNS * 9;
        last = last;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Up {
        first = COLUMNS * 8;
        last = last;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Down {
        first = COLUMNS * 10;
        last = last;
    }

    AnimationIndices {
        first: first,
        last: last,
    }
}

#[derive(Component, Clone, Default)]
pub struct CharacterAnimation {
    pub state: AnimationState,
    pub direction: AnimationDirection,
    pub animation_type: AnimationType,
}

#[derive(Component, Deref, DerefMut, Clone, Default)]
pub struct AnimationTimer(Timer);

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        _entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> PlayerBundle {
        let texture_handle = asset_server.load("sprites/alextime-1.png");
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(64.0, 64.0),
            COLUMNS,
            ROWS,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices =
            get_animation_indices(AnimationType::Walk, AnimationDirection::Right);

        let sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            ..default()
        };

        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        let collider_bundle = ColliderBundle {
            collider: Collider::cuboid(16., 27.),
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints,
            ..Default::default()
        };

        PlayerBundle {
            character_animation: CharacterAnimation { ..default() },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            sprite_bundle,
            collider_bundle,
            ..default()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
