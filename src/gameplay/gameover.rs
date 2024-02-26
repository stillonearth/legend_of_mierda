use crate::ui::UIGameOver;
use bevy::prelude::*;

#[derive(Event, Clone)]
pub struct GameOverEvent;

pub fn event_game_over(
    mut ev_game_over: EventReader<GameOverEvent>,
    mut q_ui_game_over: Query<(&mut Visibility, &UIGameOver)>,
) {
    for _ in ev_game_over.read() {
        for (mut visibility, _) in q_ui_game_over.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}
