use std::cmp::min;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    loading::load_texture_atlas, physics::ColliderBundle, sprites::PIZZA_ASSET_SHEET, ui, utils::*,
};

use super::player::Player;

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Pizza {
    pub is_dummy: bool,
}

#[derive(Clone, Default, Bundle)]
pub struct PizzaBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub pizza: Pizza,
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
}

pub fn create_pizza_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlas>,
    is_dummy: bool,
) -> PizzaBundle {
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
        PIZZA_ASSET_SHEET.to_string(),
        asset_server,
        1,
        1,
        None,
        16.,
        texture_atlasses,
    );

    let sprite_bundle = SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        ..default()
    };

    PizzaBundle {
        sprite_bundle,
        collider_bundle,
        pizza: Pizza { is_dummy },
        sensor: Sensor {},
    }
}

impl LdtkEntity for PizzaBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlas>,
    ) -> PizzaBundle {
        let is_dummy = *entity_instance
            .get_bool_field("is_dummy")
            .expect("expected entity to have non-nullable name string field");
        create_pizza_bundle(asset_server, texture_atlasses, is_dummy)
    }
}

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct PizzaStepOverEvent(pub Entity);

#[derive(Event, Clone)]
pub struct SpawnPizzaEvent {
    pub(crate) count: u32,
}

// --------------
// Event Handlers
// --------------

pub fn event_spawn_pizza(
    mut commands: Commands,
    mut ev_spawn_pizza: EventReader<SpawnPizzaEvent>,
    level_selection: Res<LevelSelection>,
    levels: Query<(Entity, &LevelIid)>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
    los_pizzas: Query<(Entity, &Parent, &Pizza)>,
    q_player_query: Query<(Entity, &Transform, &Player)>,
) {
    if q_player_query.iter().count() == 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let player_translation = q_player_query.single().1.translation;

    for ev_spawn in ev_spawn_pizza.read() {
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
                let (parent_entity, _) = levels
                    .iter()
                    .find(|(_, handle)| *handle == level_iid)
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

                        while (player_translation - mierda_position).length()
                            < max_level_dimension / 3.0
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

pub fn event_on_pizza_step_over(
    mut commands: Commands,
    mut er_pizza_step_over: EventReader<PizzaStepOverEvent>,
    mut q_pizzas: Query<(Entity, &Pizza)>,
    mut q_player: Query<(Entity, &mut Player)>,
    mut q_ui_healthbar: Query<(Entity, &mut Style, &ui::UIPlayerHealth)>,
) {
    for e in er_pizza_step_over.read() {
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

// -------
// Physics
// -------

pub(crate) fn handle_player_pizza_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_pizzas: Query<(Entity, &Pizza)>,
    q_player: Query<(Entity, &mut Player)>,
    mut ev_pizza_step_over: EventWriter<PizzaStepOverEvent>,
) {
    for (player_entity, _) in q_player.iter() {
        for event in collision_events.read() {
            for (e_pizza, _) in q_pizzas.iter_mut() {
                if let CollisionEvent::Started(e1, e2, _) = event {
                    if e1.index() == e_pizza.index() && e2.index() == player_entity.index() {
                        ev_pizza_step_over.send(PizzaStepOverEvent(e_pizza));
                        return;
                    }

                    if e2.index() == e_pizza.index() && e1.index() == player_entity.index() {
                        ev_pizza_step_over.send(PizzaStepOverEvent(e_pizza));
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

pub struct PizzaPlugin;

impl Plugin for PizzaPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PizzaBundle>("Pizza")
            // Event Handlers
            .add_event::<SpawnPizzaEvent>()
            .add_event::<PizzaStepOverEvent>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    handle_player_pizza_collision,
                    event_on_pizza_step_over,
                    event_spawn_pizza,
                ),
            );
    }
}
