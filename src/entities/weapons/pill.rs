use std::time::Duration;

use crate::entities::characters::enemy::{Enemy, EnemyType};
use crate::entities::player::{Player, PlayerHitEvent};
use crate::physics::ColliderBundle;
use crate::{loading::StaticSpriteAssets, GameState};

use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::*;

// ----------
// Components
// ----------

#[derive(Component, Clone, Copy, Default)]
pub struct RotatingPill;

#[derive(Component, Clone, Copy, Default)]
pub struct Pill;

#[derive(Component, Clone, Copy, Default)]
pub struct PillTrail;

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct PillThrowEvent {
    pub entity: Entity,
}

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct RotatingPillBundle {
    pub sprite_bundle: SpriteBundle,
    pub rotating_pill: RotatingPill,
    pub timer_activation: PillTimer,
}

#[derive(Clone, Default, Bundle)]
pub struct PillBundle {
    pub sprite_bundle: SpriteBundle,
    pub pill: Pill,
    pub timer_despawn: PillDespawnTimer,
    pub timer_trail_spawn: PillTrailSpawnTimer,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
}

#[derive(Clone, Default, Bundle)]
pub struct PillTrailBundle {
    pub sprite_bundle: SpriteBundle,
    pub pill_trail: PillTrail,
    pub timer_despawn: PillTrailDespawnTimer,
}

// ---------
// Resources
// ---------

#[derive(Resource, Default, Clone, Component)]
pub struct PillTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct PillTrailSpawnTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct PillDespawnTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct PillTrailDespawnTimer(pub Timer);

// -------
// Systems
// -------

fn inject_rotating_pill_sprite(
    mut commands: Commands,
    q_rotating_pills: Query<(&Parent, &mut Transform), With<RotatingPill>>,
    q_enemies: Query<(Entity, &Parent, &Transform, &Enemy), Without<RotatingPill>>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for (entity, _parent, _player_transform, _) in q_enemies
        .iter()
        .filter(|(_, _, _, e)| e.enemy_type == EnemyType::Psychiatrist && !e.is_dummy)
    {
        if q_rotating_pills
            .iter()
            .filter(|(p, _)| p.get() == entity)
            .count()
            != 0
        {
            continue;
        }
        let timer_activation = PillTimer(Timer::new(
            Duration::from_secs_f32(0.75),
            TimerMode::Repeating,
        ));

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                RotatingPillBundle {
                    sprite_bundle: SpriteBundle {
                        visibility: Visibility::Hidden,
                        texture: static_sprite_assets.pill.clone(),
                        transform: Transform::from_translation(Vec3::new(0.0, 0., 0.))
                            .with_scale(Vec3::ONE * 2.0),
                        ..default()
                    },
                    rotating_pill: RotatingPill,
                    timer_activation: timer_activation.clone(),
                },
                Name::new("weapon pill"),
                ZIndex::Global(403),
            ));
        });
    }
}

fn rotate_rotating_pills(
    mut q_rotating_pills: Query<(&Parent, &mut Transform), With<RotatingPill>>,
    time: Res<Time>,
) {
    let elsapsed_seconds = time.elapsed_seconds() * 3.0;
    for (_, mut transform) in q_rotating_pills.iter_mut() {
        transform.translation =
            64. * Vec3::new(elsapsed_seconds.cos(), elsapsed_seconds.sin(), 0.0);
        transform.rotation = -Quat::from_rotation_z(elsapsed_seconds * 2.);
    }
}

const TRAIL_TIMER_SPAWN_MILLIS: u64 = 10;

fn handle_pill_throw_event(
    mut commands: Commands,
    q_players: Query<(Entity, &Parent, &Transform, &Player)>,
    q_enemies: Query<(Entity, &Parent, &Transform, &Enemy)>,
    mut ev_pill_throw: EventReader<PillThrowEvent>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for pill_throw_event in ev_pill_throw.read() {
        let (pill_initial_position, parent) = q_enemies
            .get(pill_throw_event.entity)
            .map(|(_, parent, transform, _)| (transform.translation, parent.get()))
            .unwrap();

        let player_position = q_players
            .iter()
            .map(|(_, _, transform, _)| transform.translation)
            .next()
            .unwrap_or(Vec3::ZERO)
            .normalize();

        let throw_vector = player_position - pill_initial_position.normalize();

        commands.entity(parent).with_children(|parent| {
            let timer_despawn = PillDespawnTimer(Timer::new(
                Duration::from_secs_f32(1.0),
                TimerMode::Repeating,
            ));

            let timer_trail_spawn = PillTrailSpawnTimer(Timer::new(
                Duration::from_millis(TRAIL_TIMER_SPAWN_MILLIS),
                TimerMode::Repeating,
            ));

            let pill_velocity = 1500.0;
            parent.spawn((
                PillBundle {
                    sprite_bundle: SpriteBundle {
                        texture: static_sprite_assets.pill.clone(),
                        transform: Transform {
                            translation: pill_initial_position,
                            scale: Vec3::ONE * 0.5,
                            ..default()
                        },
                        ..default()
                    },
                    pill: Pill,
                    active_events: ActiveEvents::COLLISION_EVENTS,
                    timer_despawn,
                    timer_trail_spawn,
                    collider_bundle: ColliderBundle {
                        collider: Collider::cuboid(10., 5.),
                        rigid_body: RigidBody::Dynamic,
                        friction: Friction {
                            coefficient: 0.0,
                            combine_rule: CoefficientCombineRule::Min,
                        },
                        density: ColliderMassProperties::Density(105.0),
                        rotation_constraints: LockedAxes::ROTATION_LOCKED_X,
                        velocity: Velocity {
                            linvel: pill_velocity * throw_vector.truncate(),
                            angvel: 0.0,
                        },
                        ..default()
                    },
                },
                Name::new("weapon pill"),
                ZIndex::Local(202),
            ));
        });
    }
}

fn handle_pill_throw(
    q_enemies: Query<(Entity, &Enemy)>,
    mut q_pill: Query<(Entity, &Parent, &RotatingPill, &mut PillTimer)>,
    mut ev_arrow_attack: EventWriter<PillThrowEvent>,
    time: Res<Time>,
) {
    for (_, parent, _, mut timer) in q_pill.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let enemy_entity = q_enemies.get(parent.get()).unwrap().0;
            ev_arrow_attack.send(PillThrowEvent {
                entity: enemy_entity,
            });
        }
    }
}

const TRAIL_TIMER_DE_SPAWN_MILLIS: u64 = 500;

fn handle_arrow_timers(
    mut commands: Commands,
    mut q_speargun: Query<(
        Entity,
        &Parent,
        &Transform,
        &mut PillDespawnTimer,
        &mut PillTrailSpawnTimer,
        &Pill,
    )>,
    static_sprite_assets: Res<StaticSpriteAssets>,
    time: Res<Time>,
) {
    for (entity, parent, transform, mut timer_despawn, mut timer_trail, _) in q_speargun.iter_mut()
    {
        timer_despawn.0.tick(time.delta());
        timer_trail.0.tick(time.delta());
        if timer_despawn.0.just_finished() {
            commands.entity(entity).despawn_recursive();
        }

        if timer_trail.0.just_finished() {
            let timer_despawn = PillTrailDespawnTimer(Timer::new(
                Duration::from_millis(TRAIL_TIMER_DE_SPAWN_MILLIS),
                TimerMode::Once,
            ));

            commands.entity(parent.get()).with_children(|parent| {
                parent.spawn((
                    PillTrailBundle {
                        sprite_bundle: SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.3, 0.0, 0.0, 0.5),
                                ..default()
                            },
                            texture: static_sprite_assets.pill.clone(),
                            transform: *transform,
                            ..default()
                        },
                        pill_trail: PillTrail,
                        timer_despawn,
                    },
                    ZIndex::Local(105),
                    Name::new("pill trail"),
                ));
            });
        }
    }
}

fn handle_trail_timers(
    mut commands: Commands,
    mut q_arrow_trails: Query<(Entity, &mut Sprite, &mut PillTrailDespawnTimer)>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut timer) in q_arrow_trails.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }

        let opacity = (1.0 - timer.0.percent()) * 0.5;
        sprite.color = Color::rgba(0.8, 0.0, 0.0, opacity);
    }
}

// -------
// Physics
// -------

pub fn handle_pill_player_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    q_players: Query<(Entity, &Player)>,
    q_arrows: Query<(Entity, &Pill)>,
    mut ev_player_hit: EventWriter<PlayerHitEvent>,
) {
    for event in collision_events.read() {
        // println!("collision event: {:?}", event);
        if let CollisionEvent::Started(e1, e2, _) = event {
            let contact_1_player = q_players.get(*e1);
            let contact_2_player = q_players.get(*e2);
            let is_player_contact = contact_2_player.is_ok() || contact_1_player.is_ok();

            let contact_1_pill = q_arrows.get(*e1);
            let contact_2_pill = q_arrows.get(*e2);
            let is_pill_contact = contact_1_pill.is_ok() || contact_2_pill.is_ok();

            if !(is_player_contact && is_pill_contact) {
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

pub struct WeaponPillPlugin;

impl Plugin for WeaponPillPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PillTimer>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    inject_rotating_pill_sprite,
                    rotate_rotating_pills,
                    handle_pill_throw,
                    handle_pill_throw_event,
                    handle_arrow_timers,
                    handle_trail_timers,
                    handle_pill_player_collisions,
                )
                    .run_if(in_state(GameState::GamePlay)),
            )
            .add_event::<PillThrowEvent>();
    }
}
