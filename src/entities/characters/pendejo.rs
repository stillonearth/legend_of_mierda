use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;
use pecs::prelude::*;

use crate::{
    sprites::{
    AnimatedCharacterSprite, AnimationDirection, AnimationTimer, CharacterAnimation,
    },
    entities::player::Player,
    physics::ColliderBundle,
    GameState,
};

use super::enemy::{create_enemy_bundle, DirectionUpdateTime, Enemy, EnemyType};


// --------
// Entities
// --------

#[derive(Default, Bundle, Clone)]
pub struct PendejoBundle {
    pub spritesheet_bundle: SpriteSheetBundle,
    pub character_animation: CharacterAnimation,
    pub animation_timer: AnimationTimer,
    pub enemy: Enemy,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
    pub direction_update_time: DirectionUpdateTime,
    pub animated_character_sprite: AnimatedCharacterSprite,
}

// ----
// LDTK
// ----

impl LdtkEntity for PendejoBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlas>,
    ) -> PendejoBundle {
        let is_dummy = *entity_instance
            .get_bool_field("is_dummy")
            .expect("expected entity to have non-nullable name string field");

        let enemy_bundle =
            create_enemy_bundle(asset_server, texture_atlasses, is_dummy, EnemyType::Pendejo);

        PendejoBundle {
            spritesheet_bundle: enemy_bundle.spritesheet_bundle,
            character_animation: enemy_bundle.character_animation,
            animation_timer: enemy_bundle.animation_timer,
            enemy: enemy_bundle.enemy,
            collider_bundle: enemy_bundle.collider_bundle,
            active_events: enemy_bundle.active_events,
            direction_update_time: enemy_bundle.direction_update_time,
            animated_character_sprite: enemy_bundle.animated_character_sprite,
        }
    }
}

// ----------
// Pendejo AI
// ----------

pub fn pendejo_activity(time: Res<Time>, mut los_pendejos: Query<(&mut Velocity, &mut Enemy)>) {
    for (mut v, mut pendejo) in los_pendejos
        .iter_mut()
        .filter(|(_, m)| !m.is_dummy)
        .filter(|(_, m)| m.enemy_type == EnemyType::Pendejo)
    {
        let rotation_angle = time.elapsed_seconds().cos() * std::f32::consts::FRAC_PI_4;

        if pendejo.hit_at.is_some() {
            let timer = pendejo.hit_at.as_mut().unwrap();
            timer.tick(time.delta());
            if !timer.finished() {
                continue;
            } else {
                pendejo.hit_at = None;
            }
        }
        v.linvel = Vec2::new(
            pendejo.move_direction.x * rotation_angle.cos()
                - pendejo.move_direction.y * rotation_angle.sin(),
            pendejo.move_direction.x * rotation_angle.sin()
                + pendejo.move_direction.y * rotation_angle.cos(),
        ) * 30.0;
    }
}

pub fn update_pendejos_move_direction(
    time: Res<Time>,
    player: Query<(&Transform, &Player)>,
    mut los_pendejos: Query<(
        &Transform,
        &mut DirectionUpdateTime,
        &mut CharacterAnimation,
        &mut Enemy,
    )>,
) {
    if player.iter().count() == 0 {
        return;
    }

    let player_position = player.single().0.translation;

    for (mierda_position, mut direction_update_timer, mut animation, mut pendejo) in los_pendejos
        .iter_mut()
        .filter(|(_, _, _, p)| !p.is_dummy)
        .filter(|(_, _, _, p)| p.enemy_type == EnemyType::Pendejo)
    {
        direction_update_timer.timer.tick(time.delta());

        if direction_update_timer.timer.finished() || pendejo.move_direction == Vec2::ZERO {
            let mierda_position = mierda_position.translation;
            pendejo.move_direction = Vec2::new(
                player_position.x - mierda_position.x,
                player_position.y - mierda_position.y,
            )
            .normalize_or_zero();

            let angle = pendejo.move_direction.x.atan2(pendejo.move_direction.y)
                - std::f32::consts::FRAC_PI_4;

            let _degree_angle = angle * 180. / std::f32::consts::PI;

            let mut normalized_angle = angle / std::f32::consts::FRAC_PI_2;
            if normalized_angle < 0.0 {
                normalized_angle += 4.0;
            }

            animation.direction = match normalized_angle.ceil() as usize {
                4 => AnimationDirection::Up,
                1 => AnimationDirection::Right,
                2 => AnimationDirection::Down,
                3 => AnimationDirection::Left,
                _ => todo!(),
            };
        }
    }
}

// ------
// Plugin
// ------

pub struct PendejoPlugin;

impl Plugin for PendejoPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PendejoBundle>("Pendejo")
            // Event Handlers
            .add_systems(
                Update,
                (
                    // AI
                    pendejo_activity,
                    update_pendejos_move_direction,
                )
                    .run_if(in_state(GameState::GamePlay)),
            );
    }
}
