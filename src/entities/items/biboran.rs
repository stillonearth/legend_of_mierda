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

use std::f32::consts::PI;

use crate::{
    entities::characters::enemy::Enemy, physics::ColliderBundle, sprites::FlashingTimer,
    AudioAssets, GameState,
};

use crate::entities::{
    items::item::{Item, ItemStepOverEvent, ItemType},
    player::Player,
    text_indicator::SpawnTextIndicatorEvent,
};

use super::item::create_item_bundle;

// ----------
// Components
// ----------

#[derive(Component)]
pub struct BiboranSprite;

#[derive(Component)]
pub struct BiboranBookScene;

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct BiboranBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub item: Item,
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
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
        let bundle =
            create_item_bundle(asset_server, texture_atlasses, is_dummy, ItemType::Biboran);
        BiboranBundle {
            sprite_bundle: bundle.sprite_bundle,
            collider_bundle: bundle.collider_bundle,
            item: bundle.item,
            sensor: bundle.sensor,
        }
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

#[allow(clippy::single_match)]
pub fn event_on_biboran_step_over(
    mut commands: Commands,
    mut er_item_step_over: EventReader<ItemStepOverEvent>,
    mut q_items: Query<(Entity, &Item)>,
    mut q_player: Query<(Entity, &mut Player)>,
    mut q_biboran_animations: Query<(&mut Visibility, &BiboranSprite)>, // mut q_ui_healthbar: Query<(Entity, &mut Style, &ui::UIPlayerHealth)>,
    mut biboran_timer: ResMut<BiboranTimer>,
    mut biboran_effect_timer: ResMut<BiboranEffectTimer>,
    animations: Res<Animations>,
    mut players: Query<(&mut AnimationPlayer, &BiboranBookScene)>,
    audio: Res<BiboranPrayer>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for e in er_item_step_over.read() {
        if e.item_type != ItemType::Biboran {
            continue;
        }

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

        for (e_biboran, item) in q_items.iter_mut() {
            if e_biboran != e.entity {
                continue;
            }
            if item.item_type != ItemType::Biboran {
                continue;
            }
            commands.entity(e_biboran).despawn_recursive();
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

#[allow(clippy::single_match)]
fn biboran_holy_effect(
    mut commands: Commands,
    mut ev_spawn_text_indicator: EventWriter<SpawnTextIndicatorEvent>,
    mut q_biboran_sprite: Query<(&mut Visibility, &BiboranSprite)>,
    mut biboran_timer: ResMut<BiboranTimer>,
    mut biboran_effect_timer: ResMut<BiboranEffectTimer>,
    time: Res<Time>,
    audio: Res<BiboranPrayer>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut enemies: Query<(Entity, &mut Enemy)>,
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
        for (mierda_entity, mut mierda) in enemies.iter_mut() {
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

        for (enemy_entity, mut enemy) in enemies.iter_mut() {
            let damage = 5;

            let timer = Timer::new(std::time::Duration::from_millis(200), TimerMode::Once);
            enemy.hit_at = Some(timer.clone());
            enemy.health -= u8::min(damage, enemy.health);

            commands.entity(enemy_entity).insert(FlashingTimer {
                timer: timer.clone(),
            });

            ev_spawn_text_indicator.send(SpawnTextIndicatorEvent {
                text: format!("-{}", damage),
                entity: enemy_entity,
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
            .add_systems(OnEnter(GameState::GamePlay), setup_biboran_prayer)
            .add_systems(Startup, setup_biboran_scene)
            .add_systems(Update, biboran_holy_effect)
            // Event Handlers
            .add_systems(
                Update,
                (event_on_biboran_step_over, ineject_biboran_render_sprite),
            );
    }
}
