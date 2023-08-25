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

const SHEET_1_COLUMNS: usize = 13;
const SHEET_1_ROWS: usize = 21;
const SHEET_2_COLUMNS: usize = 6;
const SHEET_2_ROWS: usize = 4;
const N_FRAMES_WALK: usize = 8;
const N_FRAMES_ATTACK: usize = 5;

#[allow(dead_code)]
#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Clone, Default, Debug)]
pub enum AnimationState {
    #[default]
    Idle,
    // Run,
}

#[derive(Clone, Default, Copy, PartialEq, Debug)]
pub enum AnimationDirection {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Default, Copy, PartialEq, Debug)]
pub enum AnimationType {
    Walk,
    #[default]
    Stand,
    Attack,
}

#[derive(Resource)]
pub struct PlayerSpritesheets {
    pub player_atlas_1: Handle<TextureAtlas>,
    pub player_atlas_2: Handle<TextureAtlas>,
}

const PLAYER_ASSET_SHEET_1: &str = "sprites/alextime-1.png";
const PLAYER_ASSET_SHEET_2: &str = "sprites/alextime-2.png";

fn load_texture_atlas(
    path: &str,
    asset_server: &AssetServer,
    sheet_columns: usize,
    sheet_rows: usize,
    padding: Option<Vec2>,
    sprite_size: f32,
    texture_atlasses: &mut Assets<TextureAtlas>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(path);

    let atlas = TextureAtlas::from_grid(
        texture_handle.clone(),
        Vec2::ONE * sprite_size,
        sheet_columns,
        sheet_rows,
        padding,
        None,
    );

    let texture_handle = texture_atlasses.add(atlas);
    return texture_handle;
}

impl FromWorld for PlayerSpritesheets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let asset_server_world_borrow = world.get_resource::<AssetServer>();
        let asset_server = asset_server_world_borrow.as_deref().unwrap();

        let mut texture_atlasses_world_borrow = world.get_resource_mut::<Assets<TextureAtlas>>();
        let mut texture_atlasses = texture_atlasses_world_borrow.as_deref_mut().unwrap();

        let atlas_1 = load_texture_atlas(
            PLAYER_ASSET_SHEET_1,
            asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            None,
            64.,
            texture_atlasses,
        );

        let atlas_2 = load_texture_atlas(
            PLAYER_ASSET_SHEET_2,
            asset_server,
            SHEET_2_COLUMNS,
            SHEET_2_ROWS,
            None,
            64. * 3.,
            texture_atlasses,
        );

        PlayerSpritesheets {
            player_atlas_1: atlas_1,
            player_atlas_2: atlas_2,
        }
    }
}

pub fn get_animation_indices(
    animation_type: AnimationType,
    animation_direction: AnimationDirection,
) -> AnimationIndices {
    let mut first = 0;
    let mut last = 0;

    // Walk
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Right {
        first = SHEET_1_COLUMNS * 11 + 1;
        last = SHEET_1_COLUMNS * 11 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Left {
        first = SHEET_1_COLUMNS * 9 + 1;
        last = SHEET_1_COLUMNS * 9 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Up {
        first = SHEET_1_COLUMNS * 8 + 1;
        last = SHEET_1_COLUMNS * 8 + N_FRAMES_WALK;
    }
    if animation_type == AnimationType::Walk && animation_direction == AnimationDirection::Down {
        first = SHEET_1_COLUMNS * 10 + 1;
        last = SHEET_1_COLUMNS * 10 + N_FRAMES_WALK;
    }

    // Stand
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Right {
        first = SHEET_1_COLUMNS * 11;
        last = last;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Left {
        first = SHEET_1_COLUMNS * 9;
        last = last;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Up {
        first = SHEET_1_COLUMNS * 8;
        last = last;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Down {
        first = SHEET_1_COLUMNS * 10;
        last = last;
    }

    // Attack
    if animation_type == AnimationType::Attack && animation_direction == AnimationDirection::Right {
        first = SHEET_2_COLUMNS * 3 + 1;
        last = SHEET_2_COLUMNS * 3 + N_FRAMES_ATTACK;
    }
    if animation_type == AnimationType::Attack && animation_direction == AnimationDirection::Left {
        first = SHEET_2_COLUMNS * 1 + 1;
        last = SHEET_2_COLUMNS * 1 + N_FRAMES_ATTACK;
    }
    if animation_type == AnimationType::Attack && animation_direction == AnimationDirection::Up {
        first = SHEET_2_COLUMNS * 0 + 1;
        last = SHEET_2_COLUMNS * 0 + N_FRAMES_ATTACK;
    }
    if animation_type == AnimationType::Attack && animation_direction == AnimationDirection::Down {
        first = SHEET_2_COLUMNS * 2 + 1;
        last = SHEET_2_COLUMNS * 2 + N_FRAMES_ATTACK;
    }

    // println!("animataion_type: {:?}", animation_type);
    // println!("animataion_direction: {:?}", animation_direction);
    // println!("first: {:?}", first);
    // println!("last: {:?}", last);

    AnimationIndices {
        first: first,
        last: last,
    }
}

#[derive(Component, Clone, Default, Debug)]
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
        texture_atlasses: &mut Assets<TextureAtlas>,
    ) -> PlayerBundle {
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

        let atlas_handle = load_texture_atlas(
            PLAYER_ASSET_SHEET_1,
            asset_server,
            SHEET_1_COLUMNS,
            SHEET_1_ROWS,
            None,
            64.,
            texture_atlasses,
        );

        let sprite_bundle = SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            ..default()
        };

        PlayerBundle {
            character_animation: CharacterAnimation { ..default() },
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            sprite_bundle: sprite_bundle,
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
