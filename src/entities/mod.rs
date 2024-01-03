use bevy::prelude::*;

pub mod items;
pub mod mierda;
pub mod pendejo;
pub mod player;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        {
            let registry = app.world.resource_mut::<AppTypeRegistry>();
            let mut wr = registry.write();
            wr.register::<mierda::Mierda>();
            wr.register::<items::Pizza>();
            wr.register::<player::Player>();
        }

        app.add_plugins((
            mierda::EnemyPlugin,
            items::ItemsPlugin,
            player::PlayerPlugin,
            pendejo::PendejoPlugin,
        ));
    }
}
