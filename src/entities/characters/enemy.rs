use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use bevy_rapier2d::prelude::*;
use pecs::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;

use crate::{
    gameplay::scoring::Score, loading::load_texture_atlas, physics::ColliderBundle, sprites::*,
    utils::CloneEntity, AudioAssets, GameState,
};

use crate::entities::player::Player;
use crate::entities::text_indicator::SpawnTextIndicatorEvent;

// ----------
// Components
// ----------

#[derive(Component, Clone, Default, Reflect)]
pub struct DirectionUpdateTime {
    pub timer: Timer,
}

// --------
// Entities
// --------

#[derive(Clone, Copy, PartialEq, Debug, Default, Component, Reflect)]
pub enum EnemyType {
    #[default]
    Mierda,
    Pendejo,
}

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub move_direction: Vec2,
    pub health: u8,
    pub hit_at: Option<Timer>,
    pub is_dummy: bool,
    pub marked_for_despawn: bool,
}

#[derive(Default, Bundle, Clone)]
pub struct EnemyBundle {
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

pub fn create_enemy_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlas>,
    is_dummy: bool,
    enemy_type: EnemyType,
) -> EnemyBundle {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;

    let collider_bundle = ColliderBundle {
        collider: Collider::cuboid(8., 26.),
        rigid_body: RigidBody::Dynamic,
        friction: Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        rotation_constraints,
        ..Default::default()
    };

    let (atlas_handle, spritesheet_type) = match enemy_type {
        EnemyType::Mierda => (
            load_texture_atlas(
                MIERDA_ASSET_SHEET.to_string(),
                asset_server,
                5,
                1,
                None,
                16.,
                texture_atlasses,
            ),
            AnimatedCharacterType::NotAnimated,
        ),
        EnemyType::Pendejo => {
            let (spritesheet_path, spritesheet_type) = PENDEJO_SPRITE_SHEETS
                .choose(&mut rand::thread_rng())
                .unwrap();

            (
                load_texture_atlas(
                    spritesheet_path.to_string(),
                    asset_server,
                    SHEET_1_COLUMNS,
                    SHEET_1_ROWS,
                    None,
                    64.,
                    texture_atlasses,
                ),
                spritesheet_type.clone(),
            )
        }
        _ => panic!("Unknown enemy type"),
    };

    let sprite_bundle = SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        ..default()
    };

    let enemy = Enemy {
        health: 100,
        enemy_type,
        move_direction: Vec2 {
            x: rand::random::<f32>() * 2.0 - 1.0,
            y: rand::random::<f32>() * 2.0 - 1.0,
        }
        .normalize(),
        hit_at: None,
        is_dummy,
        marked_for_despawn: false,
    };

    EnemyBundle {
        character_animation: CharacterAnimation {
            state: AnimationState::default(),
            direction: AnimationDirection::Right,
            animation_type: AnimationType::Walk,
        },
        animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        spritesheet_bundle: sprite_bundle,
        collider_bundle,
        active_events: ActiveEvents::COLLISION_EVENTS,
        enemy: enemy,
        direction_update_time: DirectionUpdateTime {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
        },
        animated_character_sprite: AnimatedCharacterSprite {
            animated_character_type: spritesheet_type.clone(),
        },
    }
}

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct EnemyHitEvent(pub Entity);

#[derive(Event, Clone)]
pub struct SpawnEnemyEvent {
    pub count: u32,
    pub enemy_type: EnemyType,
}

// --------------
// Event Handlers
// --------------

pub fn handle_spawn_enemy(
    mut commands: Commands,
    mut ev_spawn_enemy: EventReader<SpawnEnemyEvent>,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
    enemies: Query<(Entity, &Parent, &Enemy)>,
    q_player_query: Query<(Entity, &Transform, &Player)>,
) {
    if q_player_query.iter().count() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player_translation = q_player_query.single().1.translation;

    for ev_spawn in ev_spawn_enemy.read() {
        for (_, level_iid) in levels.iter() {
            let project = project_assets.get(projects.single()).unwrap();
            let level = project.get_raw_level_by_iid(level_iid.get()).unwrap();
            let max_level_dimension = level.px_wid.max(level.px_hei) as f32;

            if level_selection.is_match(
                &LevelIndices {
                    level: 0,
                    ..default()
                },
                level,
            ) {
                for _i in 0..ev_spawn.count {
                    for (mierda_entity, enemy_parent, mierda) in enemies.iter() {
                        if !mierda.is_dummy {
                            continue;
                        }
                        if mierda.enemy_type != ev_spawn.enemy_type {
                            continue;
                        }

                        let enemy_parent = enemy_parent.get();
                        let mut parent = commands.entity(enemy_parent);

                        let mut new_entity: Option<Entity> = None;
                        parent.with_children(|cm| {
                            let ne = cm.spawn_empty().id();
                            new_entity = Some(ne);
                        });

                        // generate random position

                        let mut offset_position = Vec3::new(0.0, 0.0, 0.);
                        let mut enemy_position = player_translation + offset_position;

                        while (player_translation - enemy_position).length()
                            < max_level_dimension / 2.0
                            || enemy_position.x < 0.0 + 24.0
                            || enemy_position.x > (level.px_wid as f32) - 24.0
                            || enemy_position.y < 0.0 + 24.0
                            || enemy_position.y > (level.px_hei as f32) - 24.0
                        {
                            let r = rng.gen_range(0.0..1000.0);
                            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

                            offset_position =
                                Vec3::new(r * f32::sin(angle), r * f32::cos(angle), 0.);
                            enemy_position = player_translation + offset_position;
                        }

                        let transform =
                            Transform::from_translation(enemy_position).with_scale(Vec3::ONE * 0.5);

                        let new_entity = new_entity.unwrap();
                        commands.entity(new_entity).insert(Enemy {
                            enemy_type: ev_spawn.enemy_type,
                            is_dummy: false,
                            health: 100,
                            move_direction: Vec2::ZERO,
                            hit_at: None,
                            marked_for_despawn: false,
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

pub fn handle_enemy_hit(
    mut commands: Commands,
    q_player: Query<(&Transform, &Player)>,
    mut enemies: Query<(Entity, &Transform, &mut Velocity, &mut Enemy)>,
    mut ev_enemy_hit: EventReader<EnemyHitEvent>,
    mut ev_spawn_text_indicator: EventWriter<SpawnTextIndicatorEvent>,

    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let mut hit_sound_played = false;

    for event in ev_enemy_hit.read() {
        if commands.get_entity(event.0).is_none() {
            continue;
        }

        for (player_transform, _) in q_player.iter() {
            let player_position = player_transform.translation;

            let (enemy_entity, mierda_transform, mut enemy_velocity, mut enemy) =
                enemies.get_mut(event.0).unwrap();
            let enemy_position = mierda_transform.translation;
            let vector_attack = (enemy_position - player_position).normalize();
            enemy_velocity.linvel.x += vector_attack.x * 200.;
            enemy_velocity.linvel.y += vector_attack.y * 200.;

            let damage = match enemy.enemy_type {
                EnemyType::Mierda => 100,
                EnemyType::Pendejo => 50,
            };

            let timer = Timer::new(std::time::Duration::from_millis(200), TimerMode::Once);
            enemy.hit_at = Some(timer.clone());
            enemy.health -= u8::min(damage, enemy.health);

            if !hit_sound_played {
                audio.play(audio_assets.hit.clone()).with_volume(0.5);
                hit_sound_played = true;
            }

            commands.entity(enemy_entity).insert(FlashingTimer {
                timer: timer.clone(),
            });

            ev_spawn_text_indicator.send(SpawnTextIndicatorEvent {
                text: format!("-{}", damage),
                entity: enemy_entity,
            });
        }
    }
}

pub fn despawn_dead_enemies(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform, &mut Velocity, &mut Enemy)>,
    mut score: ResMut<Score>,
) {
    for (e, _, _, mut enemy) in enemies.iter_mut() {
        if enemy.health != 0 {
            continue;
        }

        if enemy.marked_for_despawn {
            continue;
        }

        enemy.marked_for_despawn = true;
        score.score += match enemy.enemy_type {
            EnemyType::Mierda => 100,
            EnemyType::Pendejo => 50,
        };

        commands
            .promise(|| (e))
            .then(asyn!(state => {
                state.asyn().timeout(0.3)
            }))
            .then(asyn!(state, mut commands: Commands => {
                commands.entity(state.value).despawn_recursive();
            }));
    }
}

// ------
// Plugin
// ------

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event Handlers
            .add_event::<EnemyHitEvent>()
            .add_event::<SpawnEnemyEvent>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    // Events
                    handle_enemy_hit,
                    handle_spawn_enemy,
                    // Rest
                    despawn_dead_enemies,
                )
                    .run_if(in_state(GameState::GamePlay)),
            );
    }
}
