use bevy::prelude::*;

pub mod enemies;
pub mod items;
pub mod player;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        {
            let registry = app.world.resource_mut::<AppTypeRegistry>();
            let mut wr = registry.write();
            wr.register::<enemies::Mierda>();
            wr.register::<items::Pizza>();
            wr.register::<player::Player>();
        }

        app.add_plugins((
            enemies::EnemyPlugin,
            items::ItemsPlugin,
            player::PlayerPlugin,
        ));
    }
}
