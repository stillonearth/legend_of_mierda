pub mod pill;
pub mod speargun;
pub mod weapon_arrow;

use bevy::prelude::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // weapon_arrow::WeaponArrowPlugin,
            speargun::WeaponSpeargunPlugin,
            pill::WeaponPillPlugin,
        ));
    }
}
