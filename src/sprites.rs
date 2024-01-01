use bevy::{prelude::*, sprite::Anchor};

use crate::entities::player::Player;

pub const SHEET_1_COLUMNS: usize = 13;
pub const SHEET_1_ROWS: usize = 21;
pub const SHEET_2_COLUMNS: usize = 6;
pub const SHEET_2_ROWS: usize = 4;
pub const N_FRAMES_WALK: usize = 8;
pub const N_FRAMES_ATTACK: usize = 5;

#[allow(dead_code)]
#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Clone, Default, Debug, Reflect)]
pub enum AnimationState {
    #[default]
    Idle,
    // Run,
}

#[derive(Clone, Default, Copy, PartialEq, Debug, Reflect)]
pub enum AnimationDirection {
    #[default]
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Default, Copy, PartialEq, Debug, Reflect)]
pub enum AnimationType {
    Walk,
    #[default]
    Stand,
    Attack,
}

#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct CharacterAnimation {
    pub state: AnimationState,
    pub direction: AnimationDirection,
    pub animation_type: AnimationType,
}

#[derive(Component, Reflect)]
pub struct FlashingTimer {
    pub timer: Timer,
}

#[derive(Component, Deref, DerefMut, Clone, Default, Reflect)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct PlayerSpritesheets {
    pub player_atlas_1: Handle<TextureAtlas>,
    pub player_atlas_2: Handle<TextureAtlas>,
    pub mierda_atlas: Handle<TextureAtlas>,
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

pub const PLAYER_ASSET_SHEET_1: &str = "sprites/alextime-1.png";
pub const PLAYER_ASSET_SHEET_2: &str = "sprites/alextime-2.png";
pub const MIERDA_ASSET_SHEET: &str = "sprites/mierda.png";
pub const PIZZA_ASSET_SHEET: &str = "sprites/pizza.png";

pub fn load_texture_atlas(
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
        texture_handle,
        Vec2::ONE * sprite_size,
        sheet_columns,
        sheet_rows,
        padding,
        None,
    );

    texture_atlasses.add(atlas)
}

pub fn animate_player_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut CharacterAnimation,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &mut Transform,
        ),
        With<Player>,
    >,
    spritesheets: Res<PlayerSpritesheets>,
) {
    for (mut texture_atlas, mut character_animation, mut timer, mut sprite, mut _transform) in
        &mut query
    {
        timer.tick(time.delta());

        // fix sprite position
        let mut indices = get_animation_indices(
            character_animation.animation_type,
            character_animation.direction,
        );

        if timer.just_finished() {
            sprite.index = if (sprite.index >= indices.last) || (sprite.index < indices.first) {
                // if attacking animation finished, go back to standing
                if character_animation.animation_type == AnimationType::Attack
                    && (sprite.index >= indices.last)
                {
                    character_animation.animation_type = AnimationType::Stand;
                    texture_atlas.clone_from(&spritesheets.player_atlas_1);
                }

                if character_animation.animation_type == AnimationType::Stand {
                    indices = get_animation_indices(
                        character_animation.animation_type,
                        character_animation.direction,
                    );
                }

                indices.first
            } else {
                sprite.index + 1
            };
        }

        if character_animation.animation_type == AnimationType::Walk
            || character_animation.animation_type == AnimationType::Stand
        {
            sprite.anchor = Anchor::Custom(Vec2::new(0.0, -0.12));
        } else if character_animation.animation_type == AnimationType::Attack {
            sprite.anchor = Anchor::Custom(Vec2::new(0.0, -0.05));
        }
    }
}

#[allow(clippy::erasing_op)]
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
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Left {
        first = SHEET_1_COLUMNS * 9;
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Up {
        first = SHEET_1_COLUMNS * 8;
        last = first;
    }
    if animation_type == AnimationType::Stand && animation_direction == AnimationDirection::Down {
        first = SHEET_1_COLUMNS * 10;
        last = first;
    }

    // Attack
    if animation_type == AnimationType::Attack && animation_direction == AnimationDirection::Right {
        first = SHEET_2_COLUMNS * 3;
        last = SHEET_2_COLUMNS * 3 + N_FRAMES_ATTACK;
    }
    if animation_type == AnimationType::Attack && animation_direction == AnimationDirection::Left {
        first = SHEET_2_COLUMNS;
        last = SHEET_2_COLUMNS + N_FRAMES_ATTACK;
    }
    if animation_type == AnimationType::Attack && animation_direction == AnimationDirection::Up {
        first = SHEET_2_COLUMNS * 0;
        last = SHEET_2_COLUMNS * 0 + N_FRAMES_ATTACK;
    }
    if animation_type == AnimationType::Attack && animation_direction == AnimationDirection::Down {
        first = SHEET_2_COLUMNS * 2;
        last = SHEET_2_COLUMNS * 2 + N_FRAMES_ATTACK;
    }

    AnimationIndices { first, last }
}

pub fn flash_sprite(
    mut commands: Commands,
    mut flashing_query: Query<(&mut FlashingTimer, Entity, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut timer, timer_e, mut timer_sprite) in flashing_query.iter_mut() {
        timer_sprite.color = Color::rgba(1.0, 0.0, 0.0, 0.5);

        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            timer_sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
            commands.entity(timer_e).remove::<FlashingTimer>();
        }
    }
}
