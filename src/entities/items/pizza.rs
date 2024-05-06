use std::cmp::min;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{entities::player::Player, physics::ColliderBundle, ui};

use super::item::{create_item_bundle, Item, ItemStepOverEvent, ItemType};

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Pizza {
    pub is_dummy: bool,
}

#[derive(Clone, Default, Bundle)]
pub struct PizzaBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub item: Item,
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
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
        let bundle = create_item_bundle(asset_server, texture_atlasses, is_dummy, ItemType::Pizza);

        PizzaBundle {
            sprite_bundle: bundle.sprite_bundle,
            collider_bundle: bundle.collider_bundle,
            item: bundle.item,
            sensor: bundle.sensor,
        }
    }
}

// --------------
// Event Handlers
// --------------

pub fn event_on_pizza_step_over(
    mut commands: Commands,
    mut er_item_step_over: EventReader<ItemStepOverEvent>,
    mut q_items: Query<(Entity, &Item)>,
    mut q_player: Query<(Entity, &mut Player)>,
    mut q_ui_healthbar: Query<(Entity, &mut Style, &ui::UIPlayerHealth)>,
) {
    for e in er_item_step_over.read() {
        if e.item_type != ItemType::Pizza {
            continue;
        }
        for (_, mut player) in q_player.iter_mut() {
            player.health = min(player.health + 10, 100);

            for (_, mut style, _) in q_ui_healthbar.iter_mut() {
                style.width = Val::Percent(player.health as f32);
            }
        }

        for (e_item, _) in q_items
            .iter_mut()
            .filter(|(_, i)| i.item_type == ItemType::Pizza)
        {
            if e_item != e.entity {
                continue;
            }
            commands.entity(e_item).despawn_recursive();
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
            .add_systems(Update, (event_on_pizza_step_over,));
    }
}
