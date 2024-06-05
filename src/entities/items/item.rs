use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    entities::player::Player,
    loading::load_texture_atlas,
    physics::ColliderBundle,
    sprites::{BIBORAN_ASSET_SHEET, PIZZA_ASSET_SHEET},
    utils::*,
};

#[derive(Clone, Copy, PartialEq, Debug, Default, Component, Reflect)]
pub enum ItemType {
    #[default]
    Pizza,
    Biboran,
}

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Item {
    pub is_dummy: bool,
    pub item_type: ItemType,
}

#[derive(Clone, Default, Bundle)]
pub struct ItemBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub item: Item,
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
}

pub fn create_item_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlas>,
    is_dummy: bool,
    item_type: ItemType,
) -> ItemBundle {
    let rotation_constraints = LockedAxes::ROTATION_LOCKED;

    let collider_bundle = match item_type {
        ItemType::Pizza => ColliderBundle {
            collider: Collider::cuboid(8., 8.),
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 20.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints,
            ..Default::default()
        },
        ItemType::Biboran => ColliderBundle {
            collider: Collider::cuboid(8., 16.),
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 20.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            rotation_constraints,
            ..Default::default()
        },
    };

    let atlas_handle = match item_type {
        ItemType::Pizza => load_texture_atlas(
            PIZZA_ASSET_SHEET.to_string(),
            asset_server,
            1,
            1,
            None,
            Vec2::ONE * 16.,
            texture_atlasses,
        ),
        ItemType::Biboran => load_texture_atlas(
            BIBORAN_ASSET_SHEET.to_string(),
            asset_server,
            1,
            1,
            None,
            Vec2::ONE * 32.,
            texture_atlasses,
        ),
    };

    let sprite_bundle = SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        ..default()
    };

    ItemBundle {
        sprite_bundle,
        collider_bundle,
        item: Item {
            is_dummy,
            item_type,
        },
        sensor: Sensor {},
    }
}

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct ItemStepOverEvent {
    pub entity: Entity,
    pub item_type: ItemType,
}

#[derive(Event, Clone)]
pub struct SpawnItemEvent {
    pub count: u32,
    pub item_type: ItemType,
}

// --------------
// Event Handlers
// --------------

pub fn event_spawn_item(
    mut commands: Commands,
    mut ev_spawn_item: EventReader<SpawnItemEvent>,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
    q_items: Query<(Entity, &Parent, &Item)>,
    q_player_query: Query<(Entity, &Transform, &Player)>,
) {
    if q_player_query.iter().count() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player_translation = q_player_query.single().1.translation;

    for ev_spawn in ev_spawn_item.read() {
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
                    for (item_entity, item_parent, item) in q_items.iter() {
                        if !item.is_dummy {
                            continue;
                        }

                        if item.item_type != ev_spawn.item_type {
                            continue;
                        }

                        let item_parent = item_parent.get();

                        let mut parent = commands.entity(item_parent);

                        let mut new_entity: Option<Entity> = None;
                        parent.with_children(|cm| {
                            let ne = cm.spawn_empty().id();
                            new_entity = Some(ne);
                        });

                        // generate random position

                        let mut offset_position = Vec3::new(0.0, 0.0, 0.);
                        let mut item_position = player_translation + offset_position;

                        while (player_translation - item_position).length()
                            < max_level_dimension / 3.0
                            || item_position.x < 0.0 + 24.0
                            || item_position.x > (level.px_wid as f32) - 24.0
                            || item_position.y < 0.0 + 24.0
                            || item_position.y > (level.px_hei as f32) - 24.0
                        {
                            let r = rng.gen_range(0.0..1000.0);
                            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);

                            offset_position =
                                Vec3::new(r * f32::sin(angle), r * f32::cos(angle), 0.);
                            item_position = player_translation + offset_position;
                        }

                        let transform =
                            Transform::from_translation(item_position).with_scale(Vec3::ONE * 0.5);

                        let new_entity = new_entity.unwrap();
                        commands.entity(new_entity).insert(Item {
                            is_dummy: false,
                            item_type: ev_spawn.item_type,
                        });

                        commands.add(CloneEntity {
                            source: item_entity,
                            destination: new_entity,
                        });

                        commands.entity(new_entity).insert(transform);
                        break;
                    }
                }
            }
        }
    }
}

// -------
// Physics
// -------

pub fn handle_player_item_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_items: Query<(Entity, &Item)>,
    q_player: Query<(Entity, &mut Player)>,
    mut ev_item_step_over: EventWriter<ItemStepOverEvent>,
) {
    for (player_entity, _) in q_player.iter() {
        for event in collision_events.read() {
            for (e_item, item) in q_items.iter_mut() {
                if let CollisionEvent::Started(e1, e2, _) = event {
                    if e1.index() == e_item.index() && e2.index() == player_entity.index() {
                        ev_item_step_over.send(ItemStepOverEvent {
                            entity: e_item,
                            item_type: item.item_type,
                        });

                        return;
                    }

                    if e2.index() == e_item.index() && e1.index() == player_entity.index() {
                        ev_item_step_over.send(ItemStepOverEvent {
                            entity: e_item,
                            item_type: item.item_type,
                        });

                        return;
                    }
                }
            }
        }
    }
}

// ------
// Plugin
// ------

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            // Event Handlers
            .add_event::<SpawnItemEvent>()
            .add_event::<ItemStepOverEvent>()
            // Event Handlers
            .add_systems(Update, (handle_player_item_collision, event_spawn_item));
    }
}
