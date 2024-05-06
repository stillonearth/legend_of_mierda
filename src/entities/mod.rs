use bevy::prelude::*;

pub mod characters;
pub mod items;
pub mod level_objects;
pub mod player;
pub mod text_indicator;
pub mod weapons;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        {
            let registry = app.world.resource_mut::<AppTypeRegistry>();
            let mut wr = registry.write();
            wr.register::<player::Player>();
        }

        app.add_plugins((
            characters::CharactersPlugin,
            player::PlayerPlugin,
            items::ItemsPlugin,
            weapons::weapon_arrow::WeaponArrowPlugin,
            text_indicator::TextIndicatorPlugin,
            level_objects::light::LightPlugin,
        ));
    }
}
