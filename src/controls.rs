use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use pecs::prelude::*;

use crate::{
    entities::player::{Player, PlayerAttackEvent},
    loading::CharacterSpritesheets,
    sprites::*,
};

pub fn controls(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            Entity,
            &mut TextureAtlas,
            &mut Velocity,
            &mut CharacterAnimation,
            &Player,
        ),
        With<Player>,
    >,
    spritesheets: Res<CharacterSpritesheets>,
) {
    for (entity, mut texture_atlas, mut velocity, mut char_animation, _player) in &mut query {
        // no control during attack phase
        if char_animation.animation_type == AnimationType::Attack {
            return;
        }

        if input.pressed(KeyCode::Space) {
            char_animation.animation_type = AnimationType::Attack;
            texture_atlas.layout = spritesheets.player_atlas_2.texture_atlas.layout.clone();

            let indices =
                get_animation_indices(char_animation.animation_type, char_animation.direction);
            texture_atlas.index = indices.first;
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
            let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
            let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };
            let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };

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
                    texture_atlas.layout = spritesheets.player_atlas_1.texture_atlas.layout.clone();
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
