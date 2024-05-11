use std::f64::consts::FRAC_PI_2;
use std::time::Duration;

use crate::entities::characters::enemy::{self, Enemy, EnemyHitEvent};
use crate::entities::player::Player;
use crate::physics::ColliderBundle;
use crate::{loading::StaticSpriteAssets, GameState};

use bevy::prelude::*;
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
                commands.entity(parent.get()).with_children(|parent_| {
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

                    parent_.spawn((
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

fn control(
    input: Res<Input<KeyCode>>,
    mut q_speargun: Query<(&mut Transform, &Speargun), Without<Player>>,
) {
    for (mut transform, _) in q_speargun.iter_mut() {
        let mut angle = 0.00;
        if input.pressed(KeyCode::Left) {
            angle = 0.1;
        }
        if input.pressed(KeyCode::Right) {
            angle = -0.1;
        }

        transform.rotation *= Quat::from_rotation_z(angle);
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
                            transform: transform.clone(),
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

// fn handle_speargun_attack_event(
//     mut commands: Commands,
//     mut ev_arrow_attack: EventReader<SpeargunShootEvent>,
//     q_speargun: Query<(Entity, &SpeargunArrow, &SpeargunArrowDespawnTimer)>,
// ) {
//     for _ in ev_arrow_attack.read() {
//         for (entity, _, _) in q_speargun.iter() {
//             println!("Speargun Attack!");
//         }
//     }
// }

// fn adjust_speargun_angle(
//     q_windows: Query<&Window, With<PrimaryWindow>>,
//     q_players: Query<(Entity, &Parent, &Transform, &Player)>,
//     mut paramset: ParamSet<(
//         Query<(&mut Transform, &WeaponSpeargun), Without<Player>>,
//         Query<(&mut Transform, &SpriteCamera), Without<Player>>,
//         Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
//     )>,
//     level_selection: Res<LevelSelection>,
//     projects: Query<&Handle<LdtkProject>>,
//     project_assets: Res<Assets<LdtkProject>>,
// ) {
//     let binding1 = paramset.p1();
//     let (camera_transform, _) = binding1.single();
//     let camera_position = Vec2::new(
//         camera_transform.translation.x,
//         camera_transform.translation.y,
//     );
//     let project = project_assets.get(projects.single());
//     if project.is_none() {
//         return;
//     }
//     let project = project.unwrap();

//     let mut level_position: Vec2 = Vec2::ZERO;
//     let mut level_size: Vec2 = Vec2::ZERO;

//     for (level_transform, level_iid) in paramset.p2().iter() {
//         if let Some(ldtk_level) = project.get_raw_level_by_iid(level_iid.get()) {
//             let level = &ldtk_level;
//             if level_selection.is_match(
//                 &LevelIndices {
//                     level: 0,
//                     ..default()
//                 },
//                 level,
//             ) {
//                 level_position =
//                     Vec2::new(level_transform.translation.x, level_transform.translation.y);
//                 level_size = Vec2::new(level.px_wid as f32, level.px_hei as f32);
//             }
//         }
//     }

//     if let Some(cursor_position) = q_windows.single().cursor_position() {
//         if q_players.iter().next().is_none() {
//             return;
//         }

//         let player_transform = q_players.iter().next().unwrap().2;
//         let player_position = player_transform.translation;
//         let player_position = Vec2::new(player_position.x, player_position.y);

//         println!("Player Position: {:?}", player_position);
//         println!("Cursor Position: {:?}", cursor_position);
//         println!("Camera Position: {:?}", camera_position);
//         println!("Level Position: {:?}", level_position);
//         println!(
//             "Level-Camera Position: {:?}",
//             level_position - camera_position
//         );
//         println!(
//             "Player Position-Level Size: {:?}",
//             player_position - level_size
//         );

//         let angle = {
//             let direction = cursor_position - level_size;
//             direction.angle_between(Vec2::new(1.0, 0.0))
//         };

//         println!("Angle: {}", angle);

//         // rotate the speargun
//         for (mut transform, _) in paramset.p0().iter_mut() {
//             transform.rotation = Quat::from_rotation_z(angle);
//         }
//     }
// }

// fn animate_arrow(
//     mut commands: Commands,
//     mut queries: ParamSet<(
//         Query<(&Parent, &Transform, &Player)>,
//         Query<(
//             Entity,
//             &mut Transform,
//             &mut Visibility,
//             &WeaponSpeargun,
//             &mut WeaponSpeargunTimer,
//             &mut WeaponSpeargunHideTimer,
//         )>,
//     )>,
//     mut ev_arrow_attack: EventWriter<WeaponSpeargunAttackEvent>,
//     time: Res<Time>,
// ) {
//     if queries.p0().iter().next().is_none() {
//         return;
//     }

//     for (entity, mut transform, mut visibility, arrow, mut timer_activate, mut timer_hide) in
//         queries.p1().iter_mut()
//     {
//         timer_activate.0.tick(time.delta());
//         timer_hide.0.tick(time.delta());

//         if timer_activate.0.just_finished() {
//             *visibility = Visibility::Visible;

//             timer_hide.0.unpause();

//             let end = match arrow {
//                 WeaponSpeargun::Right => Vec3::new(55., 0., 0.),
//                 WeaponSpeargun::Left => Vec3::new(-55., 0., 0.),
//             };

//             let tween = Tween::new(
//                 EaseFunction::QuadraticInOut,
//                 Duration::from_secs_f32(0.15),
//                 TransformPositionLens {
//                     start: match arrow {
//                         WeaponSpeargun::Right => Vec3::new(20., 0., 0.),
//                         WeaponSpeargun::Left => Vec3::new(-20., 0., 0.),
//                     },
//                     end,
//                 },
//             );

//             ev_arrow_attack.send(WeaponSpeargunAttackEvent {});
//             commands.entity(entity).insert(Animator::new(tween));
//         }

//         if timer_hide.0.just_finished() {
// println!("Cursor is inside the primary window, at {:?}", position);
//             timer_hide.0.pause();
//             *visibility = Visibility::Hidden;

//             transform.translation = match arrow {
//                 WeaponSpeargun::Right => Vec3::new(20., 0., 0.),
//                 WeaponSpeargun::Left => Vec3::new(-20., 0., 0.),
//             };
//         }
//     }
// }

// fn handle_arrow_attack(
//     mut arrow_attack_events: EventReader<WeaponSpeargunAttackEvent>,
//     mut ev_enemy_hit: EventWriter<EnemyHitEvent>,
//     mut queries: ParamSet<(
//         Query<(&Transform, &Player)>,
//         Query<(Entity, &Transform, &Enemy)>,
//     )>,
// ) {
//     for _ in arrow_attack_events.read() {
//         if queries.p0().iter().len() == 0 {
//             return;
//         }

//         let player_translation = queries.p0().iter().next().unwrap().0.translation;

//         for (e, transfrom, _) in queries.p1().iter() {
//             let translation = transfrom.translation;

//             if (translation.z - player_translation.z).abs() > 16.0 {
//                 continue;
//             }

//             let distance = translation.distance(player_translation).abs();

//             if distance > 40.0 {
//                 continue;
//             }

//             ev_enemy_hit.send(EnemyHitEvent(e));
//         }
//     }
// }

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
                    control,
                )
                    .run_if(in_state(GameState::GamePlay)),
            )
            .add_event::<SpeargunShootEvent>();
    }
}
