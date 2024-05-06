use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;

use crate::{
    physics::ColliderBundle,
    sprites::{AnimatedCharacterSprite, AnimationTimer, CharacterAnimation},
    GameState,
    entities::player::Player,
};

use super::{
    enemy::{create_enemy_bundle, DirectionUpdateTime, Enemy, EnemyType},
};


// -----------
// Compontents
// -----------

#[derive(Default, Bundle, Clone)]
pub struct MierdaBundle {
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

impl LdtkEntity for MierdaBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlas>,
    ) -> MierdaBundle {
        let is_dummy = *entity_instance
            .get_bool_field("is_dummy")
            .expect("expected entity to have non-nullable name string field");

        let enemy_bundle =
            create_enemy_bundle(asset_server, texture_atlasses, is_dummy, EnemyType::Mierda);

        MierdaBundle {
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

// ---------
// Mierda AI
// ---------

pub fn mierda_activity(time: Res<Time>, mut los_mierdas: Query<(&mut Velocity, &mut Enemy)>) {
    for (mut v, mut mierda) in los_mierdas
        .iter_mut()
        .filter(|(_, m)| !m.is_dummy)
        .filter(|(_, m)| m.enemy_type == EnemyType::Mierda)
    {
        let rotation_angle = time.elapsed_seconds().cos() * std::f32::consts::FRAC_PI_4;

        if mierda.hit_at.is_some() {
            let timer = mierda.hit_at.as_mut().unwrap();
            timer.tick(time.delta());
            if !timer.finished() {
                continue;
            } else {
                mierda.hit_at = None;
            }
        }
        v.linvel = Vec2::new(
            mierda.move_direction.x * rotation_angle.cos()
                - mierda.move_direction.y * rotation_angle.sin(),
            mierda.move_direction.x * rotation_angle.sin()
                + mierda.move_direction.y * rotation_angle.cos(),
        ) * 30.0;
    }
}

pub fn update_mierdas_move_direction(
    time: Res<Time>,
    player: Query<(&Transform, &Player)>,
    mut los_mierdas: Query<(&Transform, &mut DirectionUpdateTime, &mut Enemy)>,
) {
    if player.iter().count() == 0 {
        return;
    }

    let player_position = player.single().0.translation;

    for (mierda_position, mut direction_update_timer, mut mierda) in los_mierdas
        .iter_mut()
        .filter(|(_, _, m)| !m.is_dummy)
        .filter(|(_, _, m)| m.enemy_type == EnemyType::Mierda)
    {
        direction_update_timer.timer.tick(time.delta());

        if direction_update_timer.timer.finished() || mierda.move_direction == Vec2::ZERO {
            let mierda_position = mierda_position.translation;
            mierda.move_direction = Vec2::new(
                player_position.x - mierda_position.x,
                player_position.y - mierda_position.y,
            )
            .normalize_or_zero();
        }
    }
}

// ---------
// Physics
// ---------

pub fn handle_mierda_wall_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_los_mierdas: Query<(Entity, &mut Velocity, &Enemy)>,
) {
    for event in collision_events.read() {
        for (e, mut v, _) in q_los_mierdas
            .iter_mut()
            .filter(|(_, _, m)| m.enemy_type == EnemyType::Mierda)
        {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if e1.index() == e.index() || e2.index() == e.index() {
                    v.linvel *= -1.;
                }
            }
        }
    }
}

// ---
// Plugin
// --

pub struct MierdaPlugin;

impl Plugin for MierdaPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<MierdaBundle>("Mierda")
            .add_systems(
                Update,
                (
                    // AI
                    mierda_activity,
                    update_mierdas_move_direction,
                    // Physics, Collisions
                    handle_mierda_wall_collisions,
                )
                    .run_if(in_state(GameState::GamePlay)),
            );
    }
}
