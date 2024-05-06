use bevy::prelude::*;

pub mod characters;
pub mod light;
pub mod pizza;
pub mod player;
pub mod text_indicator;
pub mod weapon_arrow;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        {
            let registry = app.world.resource_mut::<AppTypeRegistry>();
            let mut wr = registry.write();
            //wr.register::<pizza::Pizza>();
            // wr.register::<biboran::Biboran>();
        }

        app.add_plugins((
            characters::CharactersPlugin,
            // pizza::PizzaPlugin,
            player::PlayerPlugin,
            // biboran::BiboranPlugin,
            weapon_arrow::WeaponArrowPlugin,
            text_indicator::TextIndicatorPlugin,
            light::LightPlugiin,
        ));
    }
}
