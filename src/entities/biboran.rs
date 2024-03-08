use std::time::Duration;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioInstance;
use bevy_rapier2d::prelude::*;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use rand::Rng;
use std::f32::consts::PI;

use crate::{
    loading::{load_texture_atlas, AudioAssets},
    physics::ColliderBundle,
    sprites::{FlashingTimer, BIBORAN_ASSET_SHEET},
    utils::*,
    GameState,
};

use super::{
    mierda::Mierda, pendejo::Pendejo, player::Player, text_indicator::SpawnTextIndicatorEvent,
};

// ----------
// Components
// ----------

#[derive(Component)]
pub struct BiboranSprite;

#[derive(Component)]
pub struct BiboranBookScene;

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Biboran {
    pub is_dummy: bool,
}

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct BiboranBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub biboran: Biboran,
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
}

pub fn create_biboran_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlas>,
    is_dummy: bool,
) -> BiboranBundle {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;

    let collider_bundle = ColliderBundle {
        collider: Collider::cuboid(8., 16.),
        rigid_body: RigidBody::Dynamic,
        friction: Friction {
            coefficient: 20.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        rotation_constraints,
        ..Default::default()
    };

    let atlas_handle = load_texture_atlas(
        BIBORAN_ASSET_SHEET.to_string(),
        asset_server,
        1,
        1,
        None,
        32.,
        texture_atlasses,
    );

    let sprite_bundle = SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        ..default()
    };

    BiboranBundle {
        sprite_bundle,
        collider_bundle,
        biboran: Biboran { is_dummy },
        sensor: Sensor {},
    }
}

impl LdtkEntity for BiboranBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlas>,
    ) -> BiboranBundle {
        let is_dummy = *entity_instance
            .get_bool_field("is_dummy")
            .expect("expected entity to have non-nullable name string field");
        create_biboran_bundle(asset_server, texture_atlasses, is_dummy)
    }
}

// ---------
// Resources
// ---------

#[derive(Resource)]
pub struct Animations(Handle<AnimationClip>);

#[derive(Resource, Default)]
pub struct BiboranRenderImage(Handle<Image>);

#[derive(Resource, Default)]
pub struct BiboranTimer(pub Timer);

#[derive(Resource, Default)]
pub struct BiboranEffectTimer(pub Timer);

#[derive(Resource, Default)]
pub struct BiboranPrayer(Handle<AudioInstance>);

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct BiboranStepOverEvent(pub Entity);

#[derive(Event, Clone)]
pub struct SpawnBiboranEvent {
    pub(crate) count: u32,
}

// --------------
// Event Handlers
// --------------

pub fn event_spawn_biboran(
    mut commands: Commands,
    mut er_spawn_biboran: EventReader<SpawnBiboranEvent>,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
    biborans: Query<(Entity, &Parent, &Biboran)>,
    q_player_query: Query<(Entity, &Transform, &Player)>,
) {
    if q_player_query.iter().count() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player_translation = q_player_query.single().1.translation;

    for ev_spawn in er_spawn_biboran.read() {
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
                    for (biboran_entity, biboran_parent, biboran) in biborans.iter() {
                        if !biboran.is_dummy {
                            continue;
                        }

                        let biboran_parent = biboran_parent.get();

                        let mut parent = commands.entity(biboran_parent);

                        let mut new_entity: Option<Entity> = None;
                        parent.with_children(|cm| {
                            let ne = cm.spawn_empty().id();
                            new_entity = Some(ne);
                        });

                        // generate random position

                        let mut offset_position = Vec3::new(0.0, 0.0, 0.);
                        let mut biboran_position = player_translation + offset_position;

                        while (player_translation - biboran_position).length()
                            < max_level_dimension / 3.0
                            || biboran_position.x < 0.0 + 24.0
                            || biboran_position.x > (level.px_wid as f32) - 24.0
                            || biboran_position.y < 0.0 + 24.0
                            || biboran_position.y > (level.px_hei as f32) - 24.0
                        {
                            let r = rng.gen_range(0.0..1000.0);
                            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

                            offset_position =
                                Vec3::new(r * f32::sin(angle), r * f32::cos(angle), 0.);
                            biboran_position = player_translation + offset_position;
                        }

                        let transform = Transform::from_translation(biboran_position)
                            .with_scale(Vec3::ONE * 0.5);

                        let new_entity = new_entity.unwrap();
                        commands
                            .entity(new_entity)
                            .insert(Biboran { is_dummy: false });

                        commands.add(CloneEntity {
                            source: biboran_entity,
                            destination: new_entity,
                        });

                        commands.entity(new_entity).insert(transform);
                    }
                }
            }
        }
    }
}

pub fn event_on_biboran_step_over(
    mut commands: Commands,
    mut er_biboran_step_over: EventReader<BiboranStepOverEvent>,
    mut q_biborans: Query<(Entity, &Biboran)>,
    mut q_player: Query<(Entity, &mut Player)>,
    mut q_biboran_animations: Query<(&mut Visibility, &BiboranSprite)>, // mut q_ui_healthbar: Query<(Entity, &mut Style, &ui::UIPlayerHealth)>,
    mut biboran_timer: ResMut<BiboranTimer>,
    mut biboran_effect_timer: ResMut<BiboranEffectTimer>,
    animations: Res<Animations>,
    mut players: Query<(&mut AnimationPlayer, &BiboranBookScene)>,
    audio: Res<BiboranPrayer>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for e in er_biboran_step_over.read() {
        for (_, mut _player) in q_player.iter_mut() {
            biboran_timer.0 = Timer::new(Duration::from_secs(14), TimerMode::Once);
            biboran_effect_timer.0 = Timer::new(Duration::from_secs(1), TimerMode::Repeating);

            for (mut v, _) in q_biboran_animations.iter_mut() {
                *v = Visibility::Visible;
            }
        }

        if let Some(instance) = audio_instances.get_mut(&audio.0) {
            match instance.state() {
                PlaybackState::Paused { .. } => {
                    instance.seek_to(0.0);
                    instance.resume(AudioTween::default());
                }
                _ => {}
            }
        }

        for (mut player, _) in &mut players {
            player.play(animations.0.clone_weak()).repeat();
        }

        for (e_biboran, _) in q_biborans.iter_mut() {
            if e_biboran != e.0 {
                continue;
            }
            commands.entity(e_biboran).despawn_recursive();
        }
    }
}

// -------
// Physics
// -------

pub(crate) fn handle_player_biboran_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_q_biborans: Query<(Entity, &Biboran)>,
    q_player: Query<(Entity, &mut Player)>,
    mut ev_biboran_step_over: EventWriter<BiboranStepOverEvent>,
) {
    for (player_entity, _) in q_player.iter() {
        for event in collision_events.read() {
            for (e_biboran, _) in q_q_biborans.iter_mut() {
                if let CollisionEvent::Started(e1, e2, _) = event {
                    if e1.index() == e_biboran.index() && e2.index() == player_entity.index() {
                        ev_biboran_step_over.send(BiboranStepOverEvent(e_biboran));
                        return;
                    }

                    if e2.index() == e_biboran.index() && e1.index() == player_entity.index() {
                        ev_biboran_step_over.send(BiboranStepOverEvent(e_biboran));
                        return;
                    }
                }
            }
        }
    }
}

// ------------
// 3D Animation
// ------------

pub fn setup_biboran_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut biboran_render_image: ResMut<BiboranRenderImage>,
) {
    // Animation
    commands.insert_resource(Animations(
        asset_server.load("models/biboran.glb#Animation0"),
    ));

    let size = Extent3d {
        width: 64,
        height: 64,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);

    let image_handle = images.add(image);
    *biboran_render_image = BiboranRenderImage(image_handle.clone());

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgba_u8(0, 0, 0, 0)),
                ..default()
            },
            ..default()
        },
        UiCameraConfig { show_ui: false },
        RenderLayers::layer(1),
    ));

    // Light
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::ZYX,
                0.0,
                1.0,
                -PI / 4.,
            )),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 200.0,
                maximum_distance: 400.0,
                ..default()
            }
            .into(),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    // Biboran
    commands.spawn((
        HookedSceneBundle {
            scene: SceneBundle {
                scene: asset_server.load("models/biboran.glb#Scene0"),
                ..default()
            },
            hook: SceneHook::new(|_, cmds| {
                cmds.insert(RenderLayers::layer(1)).insert(BiboranBookScene);
            }),
        },
        BiboranBookScene,
    ));
}

fn biboran_holy_effect(
    mut commands: Commands,
    mut ev_spawn_text_indicator: EventWriter<SpawnTextIndicatorEvent>,
    mut q_biboran_sprite: Query<(&mut Visibility, &BiboranSprite)>,
    mut biboran_timer: ResMut<BiboranTimer>,
    mut biboran_effect_timer: ResMut<BiboranEffectTimer>,
    time: Res<Time>,
    audio: Res<BiboranPrayer>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut los_mierdas: Query<(Entity, &mut Mierda)>,
    mut los_pendejos: Query<(Entity, &mut Pendejo)>,
) {
    biboran_timer.0.tick(time.delta());
    biboran_effect_timer.0.tick(time.delta());

    if biboran_timer.0.just_finished() {
        for (mut v, _) in q_biboran_sprite.iter_mut() {
            if biboran_timer.0.just_finished() {
                *v = Visibility::Hidden;
            }
        }
    }

    if !biboran_timer.0.finished() && biboran_effect_timer.0.finished() {
        for (mierda_entity, mut mierda) in los_mierdas.iter_mut() {
            let damage = 5;

            let timer = Timer::new(std::time::Duration::from_millis(200), TimerMode::Once);
            mierda.hit_at = Some(timer.clone());
            mierda.health -= u8::min(damage, mierda.health);

            commands.entity(mierda_entity).insert(FlashingTimer {
                timer: timer.clone(),
            });

            ev_spawn_text_indicator.send(SpawnTextIndicatorEvent {
                text: format!("-{}", damage),
                entity: mierda_entity,
            });
        }

        for (pendejo_entity, mut pendejo) in los_pendejos.iter_mut() {
            let damage = 5;

            let timer = Timer::new(std::time::Duration::from_millis(200), TimerMode::Once);
            pendejo.hit_at = Some(timer.clone());
            pendejo.health -= u8::min(damage, pendejo.health);

            commands.entity(pendejo_entity).insert(FlashingTimer {
                timer: timer.clone(),
            });

            ev_spawn_text_indicator.send(SpawnTextIndicatorEvent {
                text: format!("-{}", damage),
                entity: pendejo_entity,
            });
        }
    }

    if biboran_timer.0.finished() {
        if let Some(instance) = audio_instances.get_mut(&audio.0) {
            match instance.state() {
                PlaybackState::Playing { .. } => {
                    instance.pause(AudioTween::default());
                }
                _ => {}
            }
        }

        biboran_effect_timer.0.pause();
    }
}

fn ineject_biboran_render_sprite(
    mut commands: Commands,
    q_players: Query<(&Parent, &Transform, &Player)>,
    mut q_biboran_sprite: Query<(&mut Transform, &BiboranSprite), Without<Player>>,
    biboran_render_image: Res<BiboranRenderImage>,
) {
    for (parent, player_transform, _) in q_players.iter() {
        if q_biboran_sprite.iter().count() == 0 {
            commands.entity(parent.get()).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: biboran_render_image.0.clone(),
                        visibility: Visibility::Hidden,
                        transform: Transform::from_translation(
                            player_transform.translation + Vec3::new(0.0, 25.0, 0.0),
                        )
                        .with_scale(Vec3::ONE * 0.5),
                        ..default()
                    },
                    BiboranSprite,
                    Name::new("biboran animation"),
                    ZIndex::Local(101),
                ));
            });
        } else {
            for (mut biboran_transform, _) in q_biboran_sprite.iter_mut() {
                biboran_transform.translation =
                    player_transform.translation + Vec3::new(0.0, 25.0, 0.0);
            }
        }
    }
}

// -----
// Audio
// -----

fn setup_biboran_prayer(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    let handle = audio
        .play(audio_assets.biboran.clone())
        .looped()
        .with_volume(0.8)
        .handle();
    commands.insert_resource(BiboranPrayer(handle));
}

// ------
// Plugin
// ------

pub struct BiboranPlugin;

impl Plugin for BiboranPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BiboranBundle>("Biboran")
            .init_resource::<BiboranRenderImage>()
            .init_resource::<BiboranTimer>()
            .init_resource::<BiboranEffectTimer>()
            .init_resource::<BiboranPrayer>()
            // Event Handlers
            .add_event::<SpawnBiboranEvent>()
            .add_event::<BiboranStepOverEvent>()
            .add_systems(OnEnter(GameState::GamePlay), setup_biboran_prayer)
            .add_systems(Startup, setup_biboran_scene)
            .add_systems(Update, biboran_holy_effect)
            // Event Handlers
            .add_systems(
                Update,
                (
                    handle_player_biboran_collision,
                    event_on_biboran_step_over,
                    event_spawn_biboran,
                    ineject_biboran_render_sprite,
                ),
            );
    }
}
