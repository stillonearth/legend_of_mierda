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

use super::characters::enemy::{Enemy, EnemyHitEvent};

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
    mut ev_enemy_hit: EventWriter<EnemyHitEvent>,
    mut q_player: Query<(Entity, &Transform, &CharacterAnimation), With<Player>>,
    mut q_enemies: Query<(Entity, &Transform, &mut Enemy)>,
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
        for (entity, mierda_transform, _) in q_enemies.iter_mut().filter(|(_, _, m)| !m.is_dummy) {
            let mierda_position = mierda_transform.translation;

            let distance = player_position.distance(mierda_position);

            if distance >= 40. {
                continue;
            }

            // cause damage accrodign to player_orientation
            let is_enemy_attacked = match player_orientation {
                AnimationDirection::Up => player_position.y < mierda_position.y,
                AnimationDirection::Down => player_position.y > mierda_position.y,
                AnimationDirection::Left => player_position.x > mierda_position.x,
                AnimationDirection::Right => player_position.x < mierda_position.x,
            };

            if !is_enemy_attacked {
                continue;
            }

            ev_enemy_hit.send(EnemyHitEvent {
                entity,
                damage: 100,
            });
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

        if player.health == 0 {
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

// -------
// Physics
// -------

pub fn handle_player_enemy_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    q_player: Query<(Entity, &mut Player)>,
    q_enemies: Query<(Entity, &Enemy)>,
    mut ev_player_hit: EventWriter<PlayerHitEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let contact_1_player = q_player.get(*e1);
            let contact_2_player = q_player.get(*e2);
            let is_contact_player = contact_1_player.is_ok() || contact_2_player.is_ok();

            let contact_1_enemy = q_enemies.get(*e1);
            let contact_2_enemy = q_enemies.get(*e2);
            let is_contact_enemy = contact_1_enemy.is_ok() || contact_2_enemy.is_ok();

            if !(is_contact_player && is_contact_enemy) {
                continue;
            }

            let player_entity = match contact_1_player.is_ok() {
                true => contact_1_player.unwrap().0,
                false => contact_2_player.unwrap().0,
            };

            ev_player_hit.send(PlayerHitEvent {
                entity: player_entity,
            });
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
                    handle_player_enemy_collisions,
                )
                    .run_if(in_state(GameState::GamePlay)),
            );
    }
}
