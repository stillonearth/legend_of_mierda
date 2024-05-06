use bevy::prelude::*;

pub mod enemy;
pub mod mierda;
pub mod pendejo;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        {
            let registry = app.world.resource_mut::<AppTypeRegistry>();
            let mut wr = registry.write();
            wr.register::<enemy::Enemy>();
        }

        app.add_plugins((
            enemy::EnemyPlugin,
            mierda::MierdaPlugin,
            pendejo::PendejoPlugin,
        ));
    }
}
