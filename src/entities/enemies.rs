use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;
use pecs::prelude::*;
use rand::Rng;

use crate::{physics::ColliderBundle, sprites::*, utils::*};

use super::player::Player;

// -----------
// Compontents
// -----------

#[derive(Component, Clone, Default, Reflect)]
pub struct DirectionUpdateTime {
    /// track when the bomb should explode (non-repeating timer)
    pub timer: Timer,
}

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Mierda {
    pub move_direction: Vec2,
    pub health: u8,
    pub hit_at: Option<Timer>,
    pub is_dummy: bool,
}

#[derive(Clone, Default, Bundle)]
pub struct MierdaBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub mierda: Mierda,
    pub collider_bundle: ColliderBundle,
    pub direction_update_time: DirectionUpdateTime,
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
        create_mierda_bundle(asset_server, texture_atlasses, is_dummy)
    }
}

pub fn create_mierda_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlas>,
    is_dummy: bool,
) -> MierdaBundle {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;

    let collider_bundle = ColliderBundle {
        collider: Collider::cuboid(8., 8.),
        rigid_body: RigidBody::Dynamic,
        friction: Friction {
            coefficient: 20.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        rotation_constraints,
        ..Default::default()
    };

    let atlas_handle = load_texture_atlas(
        MIERDA_ASSET_SHEET,
        asset_server,
        5,
        1,
        None,
        16.,
        texture_atlasses,
    );

    let sprite_bundle = SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(4),
        ..default()
    };

    let mierda = Mierda {
        health: 100,
        move_direction: Vec2 {
            x: rand::random::<f32>() * 2.0 - 1.0,
            y: rand::random::<f32>() * 2.0 - 1.0,
        }
        .normalize(),
        hit_at: None,
        is_dummy,
    };

    MierdaBundle {
        sprite_bundle,
        collider_bundle,
        mierda,
        direction_update_time: DirectionUpdateTime {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
        },
    }
}

// ---------
// Mierda AI
// ---------

pub fn mierda_activity(time: Res<Time>, mut los_mierdas: Query<(&mut Velocity, &mut Mierda)>) {
    for (mut v, mut mierda) in los_mierdas.iter_mut().filter(|(_, m)| !m.is_dummy) {
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
    mut los_mierdas: Query<(&Transform, &mut DirectionUpdateTime, &mut Mierda)>,
) {
    if player.iter().count() == 0 {
        return;
    }

    let player_position = player.single().0.translation;

    for (mierda_position, mut direction_update_timer, mut mierda) in
        los_mierdas.iter_mut().filter(|(_, _, m)| !m.is_dummy)
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
    mut q_los_mierdas: Query<(Entity, &mut Velocity, &Mierda)>,
) {
    for event in collision_events.iter() {
        for (e, mut v, _) in q_los_mierdas.iter_mut() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if e1.index() == e.index() || e2.index() == e.index() {
                    v.linvel *= -1.;
                }
            }
        }
    }
}

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct MierdaHitEvent(pub Entity);

#[derive(Event, Clone)]
pub struct SpawnMierdaEvent {
    pub count: u32,
}

// --------------
// Event Handlers
// --------------

pub fn handle_spawn_mierda(
    mut commands: Commands,
    mut ev_spawn_mierda: EventReader<SpawnMierdaEvent>,
    level_selection: Res<LevelSelection>,
    level_handles: Query<(Entity, &Handle<LdtkLevel>)>,
    level_assets: Res<Assets<LdtkLevel>>,
    los_mierdas: Query<(Entity, &Parent, &Mierda)>,
    levels: Query<(Entity, &Handle<LdtkLevel>)>,
    q_player_query: Query<(Entity, &Transform, &Player)>,
) {
    if q_player_query.iter().count() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player_translation = q_player_query.single().1.translation;

    for ev_spawn in ev_spawn_mierda.iter() {
        for (_, level_handle) in level_handles.iter() {
            let level = &level_assets.get(level_handle).unwrap().level;

            if level_selection.is_match(&0, level) {
                let (parent_entity, _) = levels
                    .iter()
                    .find(|(_, handle)| *handle == level_handle)
                    .unwrap();

                for _i in 0..ev_spawn.count {
                    for (mierda_entity, mierda_parent, mierda) in los_mierdas.iter() {
                        if !mierda.is_dummy {
                            continue;
                        }

                        let mierda_parent = mierda_parent.get();

                        if parent_entity != mierda_parent {
                            continue;
                        }

                        let mut parent = commands.entity(mierda_parent);

                        let mut new_entity: Option<Entity> = None;
                        parent.with_children(|cm| {
                            let ne = cm.spawn_empty().id();
                            new_entity = Some(ne);
                        });

                        // generate random position

                        let mut offset_position = Vec3::new(0.0, 0.0, 0.);
                        let mut mierda_position = player_translation + offset_position;

                        while (player_translation - mierda_position).length() < 50.0
                            || mierda_position.x < 0.0 + 24.0
                            || mierda_position.x > (level.px_wid as f32) - 24.0
                            || mierda_position.y < 0.0 + 24.0
                            || mierda_position.y > (level.px_hei as f32) - 24.0
                        {
                            let x = rng.gen_range(-100.0..100.0);
                            let y = rng.gen_range(-100.0..100.0);

                            offset_position = Vec3::new(x, y, 0.);
                            mierda_position = player_translation + offset_position;
                        }

                        let transform = Transform::from_translation(mierda_position)
                            .with_scale(Vec3::ONE * 0.5);

                        let new_entity = new_entity.unwrap();
                        commands.entity(new_entity).insert(Mierda {
                            is_dummy: false,
                            health: 100,
                            move_direction: Vec2::ZERO,
                            hit_at: None,
                        });

                        commands.add(CloneEntity {
                            source: mierda_entity,
                            destination: new_entity,
                        });

                        commands.entity(new_entity).insert(transform);
                    }
                }
            }
        }
    }
}

pub fn handle_mierda_hit(
    mut commands: Commands,
    q_player: Query<(&Transform, &Player)>,
    mut los_mierdas: Query<(Entity, &Transform, &mut Velocity, &mut Mierda)>,
    mut ev_mierda_hit: EventReader<MierdaHitEvent>,
    // mut ev_mierda_spawn: EventWriter<SpawnMierdaEvent>,
) {
    for event in ev_mierda_hit.iter() {
        for (player_transform, _) in q_player.iter() {
            let player_position = player_transform.translation;

            let (mierda_entity, mierda_transform, mut mierda_velocity, mut mierda) =
                los_mierdas.get_mut(event.0).unwrap();
            let mierda_position = mierda_transform.translation;
            let vector_attack = (mierda_position - player_position).normalize();
            mierda_velocity.linvel.x += vector_attack.x * 200.;
            mierda_velocity.linvel.y += vector_attack.y * 200.;

            let timer = Timer::new(std::time::Duration::from_millis(200), TimerMode::Once);
            mierda.hit_at = Some(timer.clone());

            commands.entity(mierda_entity).insert(FlashingTimer {
                timer: timer.clone(),
            });

            // despawn mierda async
            commands
                .promise(|| (mierda_entity))
                .then(asyn!(state => {
                    state.asyn().timeout(0.3)
                }))
                .then(
                    asyn!(state, mut commands: Commands, asset_server: Res<AssetServer>, q_mierdas: Query<(Entity, &GlobalTransform)> => {
                                let mierda_transform = *q_mierdas.get(state.value).unwrap().1;
                                commands.spawn((
                                    ParticleSystemBundle {
                                        transform: (mierda_transform).into(),
                                        particle_system: ParticleSystem {
                                            spawn_rate_per_second: 0.0.into(),
                                            texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                                            max_particles: 1_00,
                                            initial_speed: (0.0..10.0).into(),
                                            scale: 1.0.into(),
                                            velocity_modifiers: vec![
                                                VelocityModifier::Drag(0.001.into()),
                                                VelocityModifier::Vector(Vec3::new(0.0, -100.0, 0.0).into()),
                                            ],
                                            color: (Color::BLUE..Color::AQUAMARINE).into(),
                                            bursts: vec![ParticleBurst {
                                                time: 0.0,
                                                count: 20,
                                            }],
                                            looping: false,
                                            ..ParticleSystem::default()
                                        },
                                        ..default()
                                    },
                                    Playing,
                                ));
                                commands.entity(state.value).despawn_recursive();
                            }),
                );
        }
    }
}

// ---
// Plugin
// --

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<MierdaBundle>("Mierda")
            // Event Handlers
            .add_event::<MierdaHitEvent>()
            .add_event::<SpawnMierdaEvent>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    // AI
                    mierda_activity,
                    update_mierdas_move_direction,
                    // Physics, Collisions
                    handle_mierda_wall_collisions,
                    // Events
                    handle_mierda_hit,
                    handle_spawn_mierda,
                ),
            );
    }
}
