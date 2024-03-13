use bevy::prelude::*;

use crate::ui::UIHighscore;

#[derive(Resource, Default)]
pub struct Score {
    pub score: u32,
}

pub fn ui_score_text(mut text_query: Query<(&mut Text, &UIHighscore)>, score: Res<Score>) {
    if !score.is_changed() {
        return;
    }

    for (mut text, _tag) in text_query.iter_mut() {
        text.sections[0].value = format!("SCORE: {}", score.score);
    }
}
