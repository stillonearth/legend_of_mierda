use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;

use crate::{
    entities::player::Player,
    physics::ColliderBundle,
    sprites::{AnimatedCharacterSprite, AnimationTimer, CharacterAnimation},
    GameState,
};

use super::enemy::{create_enemy_bundle, DirectionUpdateTime, Enemy, EnemyType};

// --------
// Entities
// --------

#[derive(Default, Bundle, Clone)]
pub struct PsychiatristBundle {
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

impl LdtkEntity for PsychiatristBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlas>,
    ) -> PsychiatristBundle {
        let is_dummy = *entity_instance
            .get_bool_field("is_dummy")
            .expect("expected entity to have non-nullable name string field");

        let mut enemy_bundle = create_enemy_bundle(
            asset_server,
            texture_atlasses,
            is_dummy,
            EnemyType::Psychiatrist,
        );

        // enemy_bundle.collider_bundle.collider = Collider::cuboid(512., 512.);

        PsychiatristBundle {
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

pub fn psychiatrist_activity(
    time: Res<Time>,
    mut q_psychiatrists: Query<(&mut Velocity, &mut Enemy)>,
) {
    for (mut v, mut psychiatrist) in q_psychiatrists
        .iter_mut()
        .filter(|(_, m)| !m.is_dummy)
        .filter(|(_, m)| m.enemy_type == EnemyType::Psychiatrist)
    {
        let rotation_angle = time.elapsed_seconds().cos() * std::f32::consts::FRAC_PI_4;

        if psychiatrist.hit_at.is_some() {
            let timer = psychiatrist.hit_at.as_mut().unwrap();
            timer.tick(time.delta());
            if !timer.finished() {
                continue;
            } else {
                psychiatrist.hit_at = None;
            }
        }
        v.linvel = Vec2::new(
            psychiatrist.move_direction.x * rotation_angle.cos()
                - psychiatrist.move_direction.y * rotation_angle.sin(),
            psychiatrist.move_direction.x * rotation_angle.sin()
                + psychiatrist.move_direction.y * rotation_angle.cos(),
        ) * 50.0;
    }
}

pub fn update_psychiatrists_move_direction(
    time: Res<Time>,
    player: Query<(&Transform, &Player)>,
    mut los_pendejos: Query<(
        &Transform,
        &mut DirectionUpdateTime,
        // &mut CharacterAnimation,
        &mut Enemy,
    )>,
) {
    if player.iter().count() == 0 {
        return;
    }

    let player_position = player.single().0.translation;

    for (psychiatrist_position, mut direction_update_timer, mut psychiatrist) in los_pendejos
        .iter_mut()
        .filter(|(_, _, p)| !p.is_dummy)
        .filter(|(_, _, p)| p.enemy_type == EnemyType::Psychiatrist)
    {
        direction_update_timer.timer.tick(time.delta());

        if direction_update_timer.timer.finished() || psychiatrist.move_direction == Vec2::ZERO {
            let mierda_position = psychiatrist_position.translation;
            psychiatrist.move_direction = Vec2::new(
                player_position.x - mierda_position.x,
                player_position.y - mierda_position.y,
            )
            .normalize_or_zero();
        }
    }
}

// ------
// Plugin
// ------

pub struct PsychiatristPlugin;

impl Plugin for PsychiatristPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PsychiatristBundle>("Psychiatrist")
            // Event Handlers
            .add_systems(
                Update,
                (
                    // AI
                    psychiatrist_activity,
                    update_psychiatrists_move_direction,
                )
                    .run_if(in_state(GameState::GamePlay)),
            );
    }
}
