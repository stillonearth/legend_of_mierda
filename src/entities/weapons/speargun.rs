use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

use crate::entities::characters::enemy::{Enemy, EnemyHitEvent};
use crate::entities::player::Player;
use crate::physics::ColliderBundle;
use crate::{loading::StaticSpriteAssets, GameState};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_magic_light_2d::SpriteCamera;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::*;

// ----------
// Components
// ----------

#[derive(Component, Clone, Copy, Default)]
pub struct Speargun;

#[derive(Component, Clone, Copy, Default)]
pub struct SpeargunArrow;

#[derive(Component, Clone, Copy, Default)]
pub struct SpeargunArrowTrail;

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct SpeargunShootEvent {}

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct SpeargunBundle {
    pub sprite_bundle: SpriteBundle,
    pub speargun: Speargun,
    pub timer_activation: SpeargunTimer,
}

#[derive(Clone, Default, Bundle)]
pub struct SpeargunArrowBundle {
    pub sprite_bundle: SpriteBundle,
    pub speargun_arrow: SpeargunArrow,
    pub timer_despawn: SpeargunArrowDespawnTimer,
    pub timer_trail_spawn: SpeargunTrailSpawnTimer,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
}

#[derive(Clone, Default, Bundle)]
pub struct SpeargunArrowTrailBundle {
    pub sprite_bundle: SpriteBundle,
    pub speargun_arrow_trail: SpeargunArrowTrail,
    pub timer_despawn: SpeargunArrowTrailDespawnTimer,
}

// ---------
// Resources
// ---------

#[derive(Resource, Default, Clone, Component)]
pub struct SpeargunTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct SpeargunTrailSpawnTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct SpeargunArrowDespawnTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct SpeargunArrowTrailDespawnTimer(pub Timer);

// -------
// Systems
// -------

fn inject_speargun_sprite(
    mut commands: Commands,
    q_players: Query<(Entity, &Parent, &Transform, &Player)>,
    mut q_spearguns: ParamSet<(Query<(&mut Transform, &Speargun), Without<Player>>,)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for (entity, _parent, _player_transform, _) in q_players.iter() {
        if q_spearguns.p0().iter().count() == 0 {
            let timer_activation = SpeargunTimer(Timer::new(
                Duration::from_secs_f32(1.0),
                TimerMode::Repeating,
            ));

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    SpeargunBundle {
                        sprite_bundle: SpriteBundle {
                            texture: static_sprite_assets.speargun.clone(),
                            transform: Transform::from_translation(Vec3::new(0.0, 0., 0.)),
                            // visibility: Visibility::Hidden,
                            ..default()
                        },
                        speargun: Speargun,
                        timer_activation: timer_activation.clone(),
                    },
                    Name::new("weapon speargun"),
                    ZIndex::Global(303),
                ));
            });
        }
    }
}

const TRAIL_TIMER_SPAWN_MILLIS: u64 = 10;

fn handle_speargun_attack_event(
    mut commands: Commands,
    q_players: Query<(Entity, &Parent, &Transform, &Player)>,
    q_spearguns: Query<(&mut Transform, &Speargun), Without<Player>>,
    mut ev_arrow_attack: EventReader<SpeargunShootEvent>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for _ in ev_arrow_attack.read() {
        for (speargun_transform, _) in q_spearguns.iter() {
            for (_, parent, player_transform, _) in q_players.iter() {
                commands.entity(parent.get()).with_children(|parent| {
                    let timer_despawn = SpeargunArrowDespawnTimer(Timer::new(
                        Duration::from_secs_f32(1.0),
                        TimerMode::Repeating,
                    ));

                    let timer_trail_spawn = SpeargunTrailSpawnTimer(Timer::new(
                        Duration::from_millis(TRAIL_TIMER_SPAWN_MILLIS),
                        TimerMode::Repeating,
                    ));

                    let z_rot = speargun_transform.rotation.to_euler(EulerRot::ZYX).0;
                    let translation = player_transform.translation
                        + 32.0 * Vec3::new(z_rot.cos(), z_rot.sin(), 0.0);
                    let arrow_velocity = 350.0;

                    parent.spawn((
                        SpeargunArrowBundle {
                            sprite_bundle: SpriteBundle {
                                texture: static_sprite_assets.speargun_arrow.clone(),
                                transform: Transform {
                                    translation,
                                    rotation: speargun_transform.rotation,
                                    ..default()
                                },
                                ..default()
                            },
                            speargun_arrow: SpeargunArrow,
                            active_events: ActiveEvents::COLLISION_EVENTS,
                            timer_despawn,
                            timer_trail_spawn,
                            collider_bundle: ColliderBundle {
                                collider: Collider::cuboid(20., 5.),
                                rigid_body: RigidBody::Dynamic,
                                friction: Friction {
                                    coefficient: 0.0,
                                    combine_rule: CoefficientCombineRule::Min,
                                },
                                density: ColliderMassProperties::Density(105.0),
                                rotation_constraints: LockedAxes::ROTATION_LOCKED_X,
                                velocity: Velocity {
                                    linvel: arrow_velocity
                                        * Vec2 {
                                            x: z_rot.cos(),
                                            y: z_rot.sin(),
                                        },
                                    angvel: 0.0,
                                },
                                ..default()
                            },
                        },
                        Name::new("weapon speargun arrow"),
                        ZIndex::Local(202),
                    ));
                });
            }
        }
    }
}

fn handle_speargun_attack(
    mut q_speargun: Query<(Entity, &Speargun, &mut SpeargunTimer)>,
    mut ev_arrow_attack: EventWriter<SpeargunShootEvent>,
    time: Res<Time>,
) {
    for (_, _, mut timer) in q_speargun.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            ev_arrow_attack.send(SpeargunShootEvent {});
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
        &mut SpeargunArrowDespawnTimer,
        &mut SpeargunTrailSpawnTimer,
        &SpeargunArrow,
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
            let timer_despawn = SpeargunArrowTrailDespawnTimer(Timer::new(
                Duration::from_millis(TRAIL_TIMER_DE_SPAWN_MILLIS),
                TimerMode::Once,
            ));

            commands.entity(parent.get()).with_children(|parent| {
                parent.spawn((
                    SpeargunArrowTrailBundle {
                        sprite_bundle: SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.3, 0.0, 0.0, 0.5),
                                ..default()
                            },
                            texture: static_sprite_assets.speargun_arrow.clone(),
                            transform: *transform,
                            ..default()
                        },
                        speargun_arrow_trail: SpeargunArrowTrail,
                        timer_despawn,
                    },
                    ZIndex::Local(105),
                    Name::new("speargun arrow trail"),
                ));
            });
        }
    }
}

fn handle_trail_timers(
    mut commands: Commands,
    mut q_arrow_trails: Query<(Entity, &mut Sprite, &mut SpeargunArrowTrailDespawnTimer)>,
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

pub fn handle_arrow_enemy_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    q_enemies: Query<(Entity, &Enemy)>,
    q_arrows: Query<(Entity, &SpeargunArrow)>,
    mut ev_enemy_hit: EventWriter<EnemyHitEvent>,
) {
    for event in collision_events.read() {
        // println!("collision event: {:?}", event);
        if let CollisionEvent::Started(e1, e2, _) = event {
            let contact_1_enemy = q_enemies.get(*e1);
            let contact_2_enemy = q_enemies.get(*e2);
            let is_enemy_contact = contact_2_enemy.is_ok() || contact_1_enemy.is_ok();

            let contact_1_arrow = q_arrows.get(*e1);
            let contact_2_arrow = q_arrows.get(*e2);
            let is_arrow_contact = contact_1_arrow.is_ok() || contact_2_arrow.is_ok();

            if !(is_enemy_contact && is_arrow_contact) {
                continue;
            }

            let enemy_entity = match contact_1_enemy.is_ok() {
                true => contact_1_enemy.unwrap().0,
                false => contact_2_enemy.unwrap().0,
            };

            ev_enemy_hit.send(EnemyHitEvent {
                entity: enemy_entity,
                damage: 50,
            });
        }
    }
}

// --------
// Controls
// --------

fn rotate_speargun(
    mut q_speargun: Query<(&mut Transform, &mut Sprite, &Speargun), Without<Player>>,
    angle: f32,
) {
    for (mut transform, mut sprite, _) in q_speargun.iter_mut() {
        transform.rotation = Quat::from_rotation_z(angle);

        sprite.flip_y = angle.abs() >= FRAC_PI_2;
    }
}

// Those are exclusive systems:
// arrows (gamepad) or mouse (keyboard)
fn control_speargun_with_arrows(
    input: Res<Input<KeyCode>>,
    q_speargun: Query<(&mut Transform, &mut Sprite, &Speargun), Without<Player>>,
) {
    if q_speargun.iter().count() == 0 {
        return;
    }

    if !(input.pressed(KeyCode::Left) || input.pressed(KeyCode::Right)) {
        return;
    }

    let mut angle = q_speargun
        .iter()
        .next()
        .unwrap()
        .0
        .rotation
        .to_euler(EulerRot::ZYX)
        .0;

    if input.pressed(KeyCode::Left) {
        angle += 0.1;
    }
    if input.pressed(KeyCode::Right) {
        angle -= -0.1;
    }

    rotate_speargun(q_speargun, angle);
}

#[allow(unused_assignments)]
fn control_speargun_with_mouse(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<SpriteCamera>>,
    q_players: Query<(Entity, &GlobalTransform, &Player)>,
    q_speargun: Query<(&mut Transform, &mut Sprite, &Speargun), Without<Player>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    let mut cursor_position = Vec2::ZERO;
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_position = world_position;
    } else {
        return;
    }

    let mut player_position = Vec2::ZERO;
    for (_, player_transform, _) in q_players.iter() {
        player_position = player_transform.translation().truncate();
    }

    let angle = {
        let direction = cursor_position - player_position;
        direction.normalize().angle_between(Vec2::new(1.0, 0.0))
    };

    rotate_speargun(q_speargun, -angle);
}

// ------
// Plugin
// ------

pub struct WeaponSpeargunPlugin;

impl Plugin for WeaponSpeargunPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpeargunTimer>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    inject_speargun_sprite,
                    handle_speargun_attack,
                    handle_speargun_attack_event,
                    handle_arrow_timers,
                    handle_trail_timers,
                    handle_arrow_enemy_collisions,
                    control_speargun_with_arrows,
                    control_speargun_with_mouse,
                )
                    .run_if(in_state(GameState::GamePlay)),
            )
            .add_event::<SpeargunShootEvent>();
    }
}
