use std::cmp::min;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};
use bevy_particle_systems::*;
use bevy_rapier2d::prelude::Velocity;
use pecs::prelude::*;

use rand::Rng;

use crate::{
    components::{Mierda, Pizza, Player},
    gameplay::{get_level_1_waves, GameplayState, WaveEntry},
    sprites::{AnimationDirection, CharacterAnimation, FlashingTimer},
    ui::{self, UIGameOver, UIGameplayWave},
    utils::CloneEntity,
};

#[derive(Event, Clone)]
pub struct PlayerAttackEvent {
    pub entity: Entity,
}

#[derive(Event, Clone)]
pub struct PlayerHitEvent {
    pub entity: Entity,
}

#[derive(Event, Clone)]
pub struct GameOverEvent;

#[derive(Event, Clone)]
pub struct MierdaHitEvent(pub Entity);

#[derive(Event, Clone)]
pub struct PizzaStepOverEvent(pub Entity);

#[derive(Event, Clone)]
pub struct SpawnMierdaEvent {
    pub(crate) count: u32,
}

#[derive(Event, Clone)]
pub struct SpawnPizzaEvent {
    pub(crate) count: u32,
}

#[derive(Event, Clone)]
pub struct LevelChangeEvent {
    pub(crate) level_id: usize,
}

pub fn event_spawn_pizza(
    mut commands: Commands,
    mut ev_spawn_pizza: EventReader<SpawnPizzaEvent>,
    level_selection: Res<LevelSelection>,
    level_handles: Query<(Entity, &Handle<LdtkLevel>)>,
    level_assets: Res<Assets<LdtkLevel>>,
    los_pizzas: Query<(Entity, &Parent, &Pizza)>,
    levels: Query<(Entity, &Handle<LdtkLevel>)>,
    q_player_query: Query<(Entity, &Transform, &Player)>,
) {
    if q_player_query.iter().count() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player_translation = q_player_query.single().1.translation;

    for ev_spawn in ev_spawn_pizza.iter() {
        for (_, level_handle) in level_handles.iter() {
            let level = &level_assets.get(level_handle).unwrap().level;

            if level_selection.is_match(&0, level) {
                let (parent_entity, _) = levels
                    .iter()
                    .find(|(_, handle)| *handle == level_handle)
                    .unwrap();

                for _i in 0..ev_spawn.count {
                    for (pizza_entity, mierda_parent, pizza) in los_pizzas.iter() {
                        if !pizza.is_dummy {
                            continue;
                        }

                        let pizza_parent = mierda_parent.get();

                        if parent_entity != pizza_parent {
                            continue;
                        }

                        let mut parent = commands.entity(pizza_parent);

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
                        commands
                            .entity(new_entity)
                            .insert(Pizza { is_dummy: false });

                        commands.add(CloneEntity {
                            source: pizza_entity,
                            destination: new_entity,
                        });

                        commands.entity(new_entity).insert(transform);
                    }
                }
            }
        }
    }
}

pub fn event_spawn_mierda(
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

pub fn event_player_attack(
    mut ev_player_attack: EventReader<PlayerAttackEvent>,
    mut ev_mierda_hit: EventWriter<MierdaHitEvent>,
    mut q_player: Query<(Entity, &Transform, &CharacterAnimation), With<Player>>,
    mut los_mierdas: Query<(Entity, &Transform, &mut Mierda)>,
) {
    for ev in ev_player_attack.iter() {
        let (_, transform, char_animation) = q_player.get_mut(ev.entity).unwrap();

        let player_position = transform.translation;
        let player_orientation = char_animation.direction;

        // find all mierdas in range
        for (entity, mierda_transform, _) in los_mierdas.iter_mut().filter(|(_, _, m)| !m.is_dummy)
        {
            let mierda_position = mierda_transform.translation;

            let distance = player_position.distance(mierda_position);

            if distance >= 75. {
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
    }
}

pub fn event_mierda_hit(
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

pub fn event_player_hit(
    mut commands: Commands,
    mut ev_player_hit_reader: EventReader<PlayerHitEvent>,
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut q_player: Query<(Entity, &GlobalTransform, &mut Player)>,
    mut q_ui_healthbar: Query<(Entity, &mut Style, &ui::UIPlayerHealth)>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_player_hit_reader.iter() {
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

        if player.health < 10 {
            ev_game_over.send(GameOverEvent);
            continue;
        } else {
            player.health -= 10;

            for (_, mut style, _) in q_ui_healthbar.iter_mut() {
                style.width = Val::Percent(player.health as f32);
            }
        }
    }
}

pub fn event_game_over(
    mut ev_game_over: EventReader<GameOverEvent>,
    mut q_ui_game_over: Query<(&mut Visibility, &UIGameOver)>,
) {
    for _ in ev_game_over.iter() {
        for (mut visibility, _) in q_ui_game_over.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}

pub fn event_on_pizza_step_over(
    mut commands: Commands,
    mut er_pizza_step_over: EventReader<PizzaStepOverEvent>,
    mut q_pizzas: Query<(Entity, &Pizza)>,
    mut q_player: Query<(Entity, &mut Player)>,
    mut q_ui_healthbar: Query<(Entity, &mut Style, &ui::UIPlayerHealth)>,
) {
    for e in er_pizza_step_over.iter() {
        for (_, mut player) in q_player.iter_mut() {
            player.health = min(player.health + 10, 100);

            for (_, mut style, _) in q_ui_healthbar.iter_mut() {
                style.width = Val::Percent(player.health as f32);
            }
        }

        for (e_pizza, _) in q_pizzas.iter_mut() {
            if e_pizza != e.0 {
                continue;
            }
            commands.entity(e_pizza).despawn_recursive();
        }
    }
}
