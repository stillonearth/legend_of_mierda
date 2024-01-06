use std::{cmp::min, time::Duration};

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
use bevy_rapier2d::prelude::*;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use rand::Rng;
use std::f32::consts::PI;

use crate::{
    loading::load_texture_atlas, physics::ColliderBundle, sprites::BIBORAN_ASSET_SHEET, utils::*,
};

use super::player::Player;

// ----------
// Components
// ----------

#[derive(Component)]
pub struct BiboranSprite;

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
    pub pizza: Biboran,
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
        BIBORAN_ASSET_SHEET,
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
        pizza: Biboran { is_dummy },
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
    mut ev_spawn_biboran: EventReader<SpawnBiboranEvent>,
    level_selection: Res<LevelSelection>,
    level_handles: Query<(Entity, &Handle<LdtkLevel>)>,
    level_assets: Res<Assets<LdtkLevel>>,
    biborans: Query<(Entity, &Parent, &Biboran)>,
    levels: Query<(Entity, &Handle<LdtkLevel>)>,
    q_player_query: Query<(Entity, &Transform, &Player)>,
) {
    if q_player_query.iter().count() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player_translation = q_player_query.single().1.translation;

    for ev_spawn in ev_spawn_biboran.iter() {
        for (_, level_handle) in level_handles.iter() {
            let level = &level_assets.get(level_handle).unwrap().level;

            if level_selection.is_match(&0, level) {
                let (parent_entity, _) = levels
                    .iter()
                    .find(|(_, handle)| *handle == level_handle)
                    .unwrap();

                for _i in 0..ev_spawn.count {
                    for (pizza_entity, mierda_parent, pizza) in biborans.iter() {
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
                            let r = rng.gen_range(0.0..1000.0);
                            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

                            offset_position =
                                Vec3::new(r * f32::sin(angle), r * f32::cos(angle), 0.);
                            mierda_position = player_translation + offset_position;
                        }

                        let transform = Transform::from_translation(mierda_position)
                            .with_scale(Vec3::ONE * 0.5);

                        let new_entity = new_entity.unwrap();
                        commands
                            .entity(new_entity)
                            .insert(Biboran { is_dummy: false });

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

pub fn event_on_biboran_step_over(
    mut commands: Commands,
    mut er_biboran_step_over: EventReader<BiboranStepOverEvent>,
    mut q_pizzas: Query<(Entity, &Biboran)>,
    mut q_player: Query<(Entity, &mut Player)>,
    mut q_biboran_animations: Query<(&mut Visibility, &BiboranSprite)>, // mut q_ui_healthbar: Query<(Entity, &mut Style, &ui::UIPlayerHealth)>,
    mut biboran_timer: ResMut<BiboranTimer>,
    animations: Res<Animations>,
    mut players: Query<(&mut AnimationPlayer)>,
) {
    for e in er_biboran_step_over.iter() {
        for (_, mut _player) in q_player.iter_mut() {
            biboran_timer.0 = Timer::new(Duration::from_secs(13), TimerMode::Once);

            for (mut v, _) in q_biboran_animations.iter_mut() {
                *v = Visibility::Visible;
            }
        }

        for (mut player) in &mut players {
            player.play(animations.0.clone_weak()).repeat();
        }

        for (e_pizza, _) in q_pizzas.iter_mut() {
            if e_pizza != e.0 {
                continue;
            }
            commands.entity(e_pizza).despawn_recursive();
        }
    }
}

// -------
// Physics
// -------

pub(crate) fn handle_player_biboran_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_pizzas: Query<(Entity, &Biboran)>,
    q_player: Query<(Entity, &mut Player)>,
    mut ev_biboran_step_over: EventWriter<BiboranStepOverEvent>,
) {
    for (player_entity, _) in q_player.iter() {
        for event in collision_events.iter() {
            for (e_pizza, _) in q_pizzas.iter_mut() {
                if let CollisionEvent::Started(e1, e2, _) = event {
                    if e1.index() == e_pizza.index() && e2.index() == player_entity.index() {
                        ev_biboran_step_over.send(BiboranStepOverEvent(e_pizza));
                        return;
                    }

                    if e2.index() == e_pizza.index() && e1.index() == player_entity.index() {
                        ev_biboran_step_over.send(BiboranStepOverEvent(e_pizza));
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
    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("models/biboran.glb#Scene0"),
            ..default()
        },
        hook: SceneHook::new(|_, cmds| {
            cmds.insert(RenderLayers::layer(1));
        }),
    });
}

fn biboran_holy_effect(
    animations: Res<Animations>,
    mut players: Query<(&mut AnimationPlayer)>,
    mut q_biboran_sprite: Query<(&mut Visibility, &BiboranSprite)>,
    mut biboran_animation_timer: ResMut<BiboranTimer>,
    time: Res<Time>,
) {
    biboran_animation_timer.0.tick(time.delta());

    if biboran_animation_timer.0.just_finished() {
        for (mut player) in &mut players {
            player.pause();
        }

        for (mut v, _) in q_biboran_sprite.iter_mut() {
            if biboran_animation_timer.0.just_finished() {
                *v = Visibility::Hidden;
            }
        }
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

// ------
// Plugin
// ------

pub struct BiboranPlugin;

impl Plugin for BiboranPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<BiboranBundle>("Biboran")
            .init_resource::<BiboranRenderImage>()
            .init_resource::<BiboranTimer>()
            // Event Handlers
            .add_event::<SpawnBiboranEvent>()
            .add_event::<BiboranStepOverEvent>()
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
