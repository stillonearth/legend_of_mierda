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

use super::{player::Player, text_indicator::SpawnTextIndicatorEvent};

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

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Pendejo {
    pub move_direction: Vec2,
    pub health: u8,
    pub hit_at: Option<Timer>,
    pub is_dummy: bool,
    pub marked_for_despawn: bool,
}

#[derive(Default, Bundle)]
pub struct PendejoBundle {
    pub spritesheet_bundle: SpriteSheetBundle,
    pub character_animation: CharacterAnimation,
    pub animation_timer: AnimationTimer,
    pub pendejo: Pendejo,
    pub collider_bundle: ColliderBundle,
    pub active_events: ActiveEvents,
    pub direction_update_time: DirectionUpdateTime,
    pub animated_character_sprite: AnimatedCharacterSprite,
}

// ----
// LDTK
// ----

pub fn create_pendejo_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlasLayout>,
    is_dummy: bool,
) -> PendejoBundle {
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

    let (spritesheet_path, spritesheet_type) = PENDEJO_SPRITE_SHEETS
        .choose(&mut rand::thread_rng())
        .unwrap();

    let atlas_image_bundle = load_texture_atlas(
        spritesheet_path.to_string(),
        asset_server,
        SHEET_1_COLUMNS,
        SHEET_1_ROWS,
        None,
        64.,
        texture_atlasses,
    );

    let sprite_bundle = SpriteSheetBundle {
        atlas: TextureAtlas {
            layout: atlas_image_bundle.texture_atlas.layout,
            index: 0,
        },
        sprite: Sprite::default(),
        ..default()
    };

    let pendejo = Pendejo {
        health: 100,
        move_direction: Vec2 {
            x: rand::random::<f32>() * 2.0 - 1.0,
            y: rand::random::<f32>() * 2.0 - 1.0,
        }
        .normalize(),
        hit_at: None,
        is_dummy,
        marked_for_despawn: false,
    };

    PendejoBundle {
        character_animation: CharacterAnimation {
            state: AnimationState::default(),
            direction: AnimationDirection::Right,
            animation_type: AnimationType::Walk,
        },
        animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        spritesheet_bundle: sprite_bundle,
        collider_bundle,
        active_events: ActiveEvents::COLLISION_EVENTS,
        pendejo,
        direction_update_time: DirectionUpdateTime {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
        },
        animated_character_sprite: AnimatedCharacterSprite {
            animated_character_type: *spritesheet_type,
        },
    }
}

impl LdtkEntity for PendejoBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlasLayout>,
    ) -> PendejoBundle {
        let is_dummy = *entity_instance
            .get_bool_field("is_dummy")
            .expect("expected entity to have non-nullable name string field");
        create_pendejo_bundle(asset_server, texture_atlasses, is_dummy)
    }
}

// ---------
// Mierda AI
// ---------

pub fn pendejo_activity(time: Res<Time>, mut los_pendejos: Query<(&mut Velocity, &mut Pendejo)>) {
    for (mut v, mut pendejo) in los_pendejos.iter_mut().filter(|(_, m)| !m.is_dummy) {
        let rotation_angle = time.elapsed_seconds().cos() * std::f32::consts::FRAC_PI_4;

        if pendejo.hit_at.is_some() {
            let timer = pendejo.hit_at.as_mut().unwrap();
            timer.tick(time.delta());
            if !timer.finished() {
                continue;
            } else {
                pendejo.hit_at = None;
            }
        }
        v.linvel = Vec2::new(
            pendejo.move_direction.x * rotation_angle.cos()
                - pendejo.move_direction.y * rotation_angle.sin(),
            pendejo.move_direction.x * rotation_angle.sin()
                + pendejo.move_direction.y * rotation_angle.cos(),
        ) * 30.0;
    }
}

pub fn update_pendejos_move_direction(
    time: Res<Time>,
    player: Query<(&Transform, &Player)>,
    mut los_pendejos: Query<(
        &Transform,
        &mut DirectionUpdateTime,
        &mut CharacterAnimation,
        &mut Pendejo,
    )>,
) {
    if player.iter().count() == 0 {
        return;
    }

    let player_position = player.single().0.translation;

    for (mierda_position, mut direction_update_timer, mut animation, mut pendejo) in
        los_pendejos.iter_mut().filter(|(_, _, _, p)| !p.is_dummy)
    {
        direction_update_timer.timer.tick(time.delta());

        if direction_update_timer.timer.finished() || pendejo.move_direction == Vec2::ZERO {
            let mierda_position = mierda_position.translation;
            pendejo.move_direction = Vec2::new(
                player_position.x - mierda_position.x,
                player_position.y - mierda_position.y,
            )
            .normalize_or_zero();

            let angle = pendejo.move_direction.x.atan2(pendejo.move_direction.y)
                - std::f32::consts::FRAC_PI_4;

            let _degree_angle = angle * 180. / std::f32::consts::PI;

            let mut normalized_angle = angle / std::f32::consts::FRAC_PI_2;
            if normalized_angle < 0.0 {
                normalized_angle += 4.0;
            }

            animation.direction = match normalized_angle.ceil() as usize {
                4 => AnimationDirection::Up,
                1 => AnimationDirection::Right,
                2 => AnimationDirection::Down,
                3 => AnimationDirection::Left,
                _ => todo!(),
            };
        }
    }
}

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct PendejoHitEvent(pub Entity);

#[derive(Event, Clone)]
pub struct SpawnPendejoEvent {
    pub count: u32,
}

// --------------
// Event Handlers
// --------------

pub fn handle_spawn_pendejo(
    mut commands: Commands,
    mut ev_spawn_mierda: EventReader<SpawnPendejoEvent>,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
    los_pendejos: Query<(Entity, &Parent, &Pendejo)>,
    q_player_query: Query<(Entity, &Transform, &Player)>,
) {
    if q_player_query.iter().count() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player_translation = q_player_query.single().1.translation;

    for ev_spawn in ev_spawn_mierda.read() {
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
                    for (mierda_entity, mierda_parent, mierda) in los_pendejos.iter() {
                        if !mierda.is_dummy {
                            continue;
                        }

                        let mierda_parent = mierda_parent.get();

                        let mut parent = commands.entity(mierda_parent);

                        let mut new_entity: Option<Entity> = None;
                        parent.with_children(|cm| {
                            let ne = cm.spawn_empty().id();
                            new_entity = Some(ne);
                        });

                        // generate random position

                        let mut offset_position = Vec3::new(0.0, 0.0, 0.);
                        let mut pendejo_position = player_translation + offset_position;

                        while (player_translation - pendejo_position).length()
                            < max_level_dimension / 2.0
                            || pendejo_position.x < 0.0 + 24.0
                            || pendejo_position.x > (level.px_wid as f32) - 24.0
                            || pendejo_position.y < 0.0 + 24.0
                            || pendejo_position.y > (level.px_hei as f32) - 24.0
                        {
                            let r = rng.gen_range(0.0..1000.0);
                            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

                            offset_position =
                                Vec3::new(r * f32::sin(angle), r * f32::cos(angle), 0.);
                            pendejo_position = player_translation + offset_position;
                        }

                        let transform = Transform::from_translation(pendejo_position)
                            .with_scale(Vec3::ONE * 0.5);

                        let new_entity = new_entity.unwrap();
                        commands.entity(new_entity).insert(Pendejo {
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

                        // let ta = texture_atlasses.as_mut();
                        // let pendejo_bundle = create_pendejo_bundle(&asset_server, ta, false);

                        commands.entity(new_entity).insert(transform);
                    }
                }
            }
        }
    }
}

pub fn handle_pendejo_hit(
    mut commands: Commands,
    q_player: Query<(&Transform, &Player)>,
    mut los_pendejos: Query<(Entity, &Transform, &mut Velocity, &mut Pendejo)>,
    mut ev_pendejo_hit: EventReader<PendejoHitEvent>,
    mut ev_spawn_text_indicator: EventWriter<SpawnTextIndicatorEvent>,

    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let mut hit_sound_played = false;

    for event in ev_pendejo_hit.read() {
        if commands.get_entity(event.0).is_none() {
            continue;
        }

        for (player_transform, _) in q_player.iter() {
            let player_position = player_transform.translation;

            let (mierda_entity, mierda_transform, mut mierda_velocity, mut mierda) =
                los_pendejos.get_mut(event.0).unwrap();
            let mierda_position = mierda_transform.translation;
            let vector_attack = (mierda_position - player_position).normalize();
            mierda_velocity.linvel.x += vector_attack.x * 200.;
            mierda_velocity.linvel.y += vector_attack.y * 200.;

            // let distance = mierda_position.distance(player_position).abs();
            let damage = 50;

            let timer = Timer::new(std::time::Duration::from_millis(200), TimerMode::Once);
            mierda.hit_at = Some(timer.clone());
            mierda.health -= u8::min(damage, mierda.health);

            if !hit_sound_played {
                audio.play(audio_assets.hit.clone()).with_volume(0.5);
                hit_sound_played = true;
            }

            commands.entity(mierda_entity).insert(FlashingTimer {
                timer: timer.clone(),
            });

            ev_spawn_text_indicator.send(SpawnTextIndicatorEvent {
                text: format!("-{}", damage),
                entity: mierda_entity,
            });
        }
    }
}

pub fn despawn_dead_pendejos(
    mut commands: Commands,
    mut los_mierdas: Query<(Entity, &Transform, &mut Velocity, &mut Pendejo)>,
    mut score: ResMut<Score>,
) {
    for (e, _, _, mut m) in los_mierdas.iter_mut() {
        if m.health != 0 {
            continue;
        }

        if m.marked_for_despawn {
            continue;
        }

        m.marked_for_despawn = true;
        score.score += 100;

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

pub struct PendejoPlugin;

impl Plugin for PendejoPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PendejoBundle>("Pendejo")
            // Event Handlers
            .add_event::<PendejoHitEvent>()
            .add_event::<SpawnPendejoEvent>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    // AI
                    pendejo_activity,
                    update_pendejos_move_direction,
                    // Events
                    handle_pendejo_hit,
                    handle_spawn_pendejo,
                    // Rest
                    despawn_dead_pendejos,
                )
                    .run_if(in_state(GameState::GamePlay)),
            );
    }
}
