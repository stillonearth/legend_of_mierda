use bevy::prelude::*;

pub mod biboran;
pub mod light;
pub mod mierda;
pub mod pendejo;
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
            wr.register::<mierda::Mierda>();
            wr.register::<pizza::Pizza>();
            wr.register::<player::Player>();
            wr.register::<biboran::Biboran>();
        }

        app.add_plugins((
            mierda::EnemyPlugin,
            pizza::PizzaPlugin,
            player::PlayerPlugin,
            pendejo::PendejoPlugin,
            biboran::BiboranPlugin,
            weapon_arrow::WeaponArrowPlugin,
            text_indicator::TextIndicatorPlugin,
            light::LightPlugiin,
        ));
    }
}
