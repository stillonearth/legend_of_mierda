use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use pecs::prelude::*;

use crate::{
    entities::player::{Player, PlayerAttackEvent},
    loading::CharacterSpritesheets,
    sprites::*,
};

#[derive(Event, Copy, Clone, Reflect, Debug, PartialEq, Eq, Default)]
pub struct ControlEvent {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub attack: bool,
}

pub fn control_character(
    mut commands: Commands,
    mut ev_control: EventReader<ControlEvent>,
    mut query: Query<
        (
            Entity,
            &mut Handle<TextureAtlas>,
            &mut Velocity,
            &mut CharacterAnimation,
            &mut TextureAtlasSprite,
            &Player,
        ),
        With<Player>,
    >,
    spritesheets: Res<CharacterSpritesheets>,
) {
    for control in ev_control.read() {
        for (entity, mut texture_atlas, mut velocity, mut char_animation, mut sprite, _player) in
            &mut query
        {
            // no control during attack phase
            if char_animation.animation_type == AnimationType::Attack {
                return;
            }

            if control.attack {
                char_animation.animation_type = AnimationType::Attack;
                texture_atlas.clone_from(&spritesheets.player_atlas_2);

                let indices =
                    get_animation_indices(char_animation.animation_type, char_animation.direction);
                sprite.index = indices.first;
                velocity.linvel = Vec2::ZERO;

                commands
                    .promise(|| (entity))
                    .then(asyn!(state => {
                        state.asyn().timeout(0.3)
                    }))
                    .then(
                        asyn!(state, mut ev_attack: EventWriter<PlayerAttackEvent> => {
                                    let event = PlayerAttackEvent { entity: state.value };
                        ev_attack.send(event);
                                }),
                    );
            } else {
                let right = if control.right { 1. } else { 0. };
                let left = if control.left { 1. } else { 0. };
                let up = if control.up { 1. } else { 0. };
                let down = if control.down { 1. } else { 0. };

                velocity.linvel.x = right - left;
                velocity.linvel.y = up - down;

                velocity.linvel = velocity.linvel.normalize_or_zero() * 100.;

                let linvel_norm = velocity.linvel.distance(Vec2::ZERO);

                // Change animation type if player moved
                if char_animation.animation_type == AnimationType::Walk {
                    if velocity.linvel.x > 0. {
                        char_animation.direction = AnimationDirection::Right;
                    } else if velocity.linvel.x < 0. {
                        char_animation.direction = AnimationDirection::Left;
                    } else if velocity.linvel.y > 0. {
                        char_animation.direction = AnimationDirection::Up;
                    } else if velocity.linvel.y < 0. {
                        char_animation.direction = AnimationDirection::Down;
                    }
                }

                // Don't interrupt attack animation
                if char_animation.animation_type != AnimationType::Attack {
                    // Change spritesheet
                    if char_animation.animation_type != AnimationType::Walk {
                        texture_atlas.clone_from(&spritesheets.player_atlas_1);
                    }

                    if linvel_norm == 0.0 {
                        char_animation.animation_type = AnimationType::Stand;
                    } else {
                        char_animation.animation_type = AnimationType::Walk;
                    }
                }
            }
        }
    }
}

pub fn keyboard_controls(input: Res<Input<KeyCode>>, mut ev_control: EventWriter<ControlEvent>) {
    let mut control = ControlEvent { ..default() };

    control.right = input.pressed(KeyCode::D);
    control.left = input.pressed(KeyCode::A);
    control.up = input.pressed(KeyCode::W);
    control.down = input.pressed(KeyCode::S);

    // if input.pressed(KeyCode::Space) {
    //     // control.attack = true;
    // } else {

    // }

    ev_control.send(control);
}
