use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;

use crate::{
    gameover::GameOverEvent, loading::load_texture_atlas, physics::ColliderBundle, sprites::*,
    ui::UIPlayerHealth, AudioAssets, GameState,
};

use super::{
    mierda::{Mierda, MierdaHitEvent},
    pendejo::{Pendejo, PendejoHitEvent},
};

// --------
// Entities
// --------

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
pub struct Player {
    pub health: u8,
}

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub character_animation: CharacterAnimation,
    pub animation_timer: AnimationTimer,
    pub player: Player,
    pub animated_character_sprite: AnimatedCharacterSprite,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
    pub name: Name,
}

// ----
// LDTK
// ----

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
            collider: Collider::cuboid(8., 26.),
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints,
            density: ColliderMassProperties::Mass(300.0),
            ..Default::default()
        };

        let atlas_handle = load_texture_atlas(
            PLAYER_ASSET_SHEET_1.to_string(),
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
            sprite_bundle,
            collider_bundle,
            active_events: ActiveEvents::COLLISION_EVENTS,
            player: Player { health: 100 },
            animated_character_sprite: AnimatedCharacterSprite {
                animated_character_type: AnimatedCharacterType::Player,
            },
            name: Name::new("Player"),
        }
    }
}

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct PlayerAttackEvent {
    pub entity: Entity,
}

#[derive(Event, Clone)]
pub struct PlayerHitEvent {
    pub entity: Entity,
}

// --------------
// Event Handlers
// --------------

pub fn event_player_attack(
    mut commands: Commands,
    mut ev_player_attack: EventReader<PlayerAttackEvent>,
    mut ev_mierda_hit: EventWriter<MierdaHitEvent>,
    mut ev_pendejo_hit: EventWriter<PendejoHitEvent>,
    mut q_player: Query<(Entity, &Transform, &CharacterAnimation), With<Player>>,
    mut q_los_mierdas: Query<(Entity, &Transform, &mut Mierda)>,
    mut q_los_pendejos: Query<(Entity, &Transform, &mut Pendejo)>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for ev in ev_player_attack.read() {
        if commands.get_entity(ev.entity).is_none() {
            continue;
        }

        let (_, transform, char_animation) = q_player.get_mut(ev.entity).unwrap();

        let player_position = transform.translation;
        let player_orientation = char_animation.direction;

        audio.play(audio_assets.slash.clone());

        // find all mierdas in range
        for (entity, mierda_transform, _) in
            q_los_mierdas.iter_mut().filter(|(_, _, m)| !m.is_dummy)
        {
            let mierda_position = mierda_transform.translation;

            let distance = player_position.distance(mierda_position);

            if distance >= 40. {
                continue;
            }

            // cause damage accrodign to player_orientation
            let is_merda_attacked = match player_orientation {
                AnimationDirection::Up => player_position.y < mierda_position.y,
                AnimationDirection::Down => player_position.y > mierda_position.y,
                AnimationDirection::Left => player_position.x > mierda_position.x,
                AnimationDirection::Right => player_position.x < mierda_position.x,
            };

            if !is_merda_attacked {
                continue;
            }

            ev_mierda_hit.send(MierdaHitEvent(entity));
        }

        // same for pendejos
        for (entity, pendejo_transform, _) in
            q_los_pendejos.iter_mut().filter(|(_, _, m)| !m.is_dummy)
        {
            let pendejo_position = pendejo_transform.translation;

            let distance = player_position.distance(pendejo_position);

            if distance >= 40. {
                continue;
            }

            // cause damage accrodign to player_orientation
            let is_pendejo_attacked = match player_orientation {
                AnimationDirection::Up => player_position.y < pendejo_position.y,
                AnimationDirection::Down => player_position.y > pendejo_position.y,
                AnimationDirection::Left => player_position.x > pendejo_position.x,
                AnimationDirection::Right => player_position.x < pendejo_position.x,
            };

            if !is_pendejo_attacked {
                continue;
            }

            ev_pendejo_hit.send(PendejoHitEvent(entity));
        }
    }
}

pub fn event_player_hit(
    mut commands: Commands,
    mut ev_player_hit_reader: EventReader<PlayerHitEvent>,
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut q_player: Query<(Entity, &GlobalTransform, &mut Player)>,
    mut q_ui_healthbar: Query<(Entity, &mut Style, &UIPlayerHealth)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for ev in ev_player_hit_reader.read() {
        if commands.get_entity(ev.entity).is_none() {
            continue;
        }

        let (_, player_transform, mut player) = q_player.get_mut(ev.entity).unwrap();

        commands.spawn((
            ParticleSystemBundle {
                transform: (*player_transform).into(),
                particle_system: ParticleSystem {
                    spawn_rate_per_second: 0.0.into(),
                    texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                    max_particles: 5_000,
                    initial_speed: (0.0..300.0).into(),
                    scale: 1.0.into(),
                    velocity_modifiers: vec![
                        VelocityModifier::Drag(0.001.into()),
                        VelocityModifier::Vector(Vec3::new(0.0, -400.0, 0.0).into()),
                    ],
                    color: (Color::RED..Color::rgba(1.0, 0.0, 0.0, 0.0)).into(),
                    bursts: vec![ParticleBurst {
                        time: 0.0,
                        count: 1000,
                    }],
                    looping: false,
                    ..ParticleSystem::default()
                },
                ..default()
            },
            Playing,
        ));

        audio.play(audio_assets.hurt.clone()).with_volume(0.5);

        if player.health <= 0 {
            ev_game_over.send(GameOverEvent);
            continue;
        } else {
            player.health -= 1;

            for (_, mut style, _) in q_ui_healthbar.iter_mut() {
                style.width = Val::Percent(player.health as f32);
            }
        }
    }
}

// ---------
// Physics
// ---------

pub fn handle_player_mierda_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_player: Query<(Entity, &mut Player)>,
    q_los_mierdas: Query<(Entity, &mut Velocity, &Mierda)>,
    mut ev_player_hit: EventWriter<PlayerHitEvent>,
) {
    for event in collision_events.read() {
        for (e, _) in q_player.iter_mut() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if !(e1.index() == e.index() || e2.index() == e.index()) {
                    continue;
                }

                let other_entity = if e1.index() == e.index() { *e2 } else { *e1 };
                if q_los_mierdas.get(other_entity).is_err() {
                    continue;
                }

                ev_player_hit.send(PlayerHitEvent { entity: e });
            }
        }
    }
}

pub fn handle_player_pendejo_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_player: Query<(Entity, &mut Player)>,
    q_los_pendejos: Query<(Entity, &mut Velocity, &Pendejo)>,
    mut ev_player_hit: EventWriter<PlayerHitEvent>,
) {
    for event in collision_events.read() {
        for (e, _) in q_player.iter_mut() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if !(e1.index() == e.index() || e2.index() == e.index()) {
                    continue;
                }

                let other_entity = if e1.index() == e.index() { *e2 } else { *e1 };
                if q_los_pendejos.get(other_entity).is_err() {
                    continue;
                }

                ev_player_hit.send(PlayerHitEvent { entity: e });
            }
        }
    }
}

// ------
// Plugin
// ------

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            // Events
            .add_event::<PlayerAttackEvent>()
            .add_event::<PlayerHitEvent>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    event_player_attack,
                    event_player_hit,
                    handle_player_mierda_collisions,
                    handle_player_pendejo_collisions,
                )
                    .run_if(in_state(GameState::GamePlay)),
            );
    }
}
