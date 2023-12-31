use bevy::prelude::*;

pub mod gameover;
pub mod waves;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<waves::GameplayState>()
            .add_systems(
                Update,
                (
                    waves::event_on_level_change,
                    waves::event_wave,
                    waves::ui_wave_info_text,
                    waves::handle_timers,
                ),
            )
            // Handle game over
            .add_systems(Update, (gameover::event_game_over,))
            .add_event::<gameover::GameOverEvent>()
            .add_event::<waves::WaveEvent>();
    }
}
