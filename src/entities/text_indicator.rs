use bevy::prelude::*;

use crate::loading::FontAssets;

// -----------
// Compontents
// -----------

#[derive(Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
pub struct TextIndicator {
    pub timer: Timer,
}

// --------
// Systems
// --------

pub fn update_text_indicator(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TextIndicator)>,
) {
    for (entity, mut transform, mut text_indicator) in query.iter_mut() {
        text_indicator.timer.tick(time.delta());

        transform.translation.y += 50.0 * f32::sin(time.delta().as_secs_f32());

        if text_indicator.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct SpawnTextIndicatorEvent {
    pub text: String,
    pub entity: Entity,
}

// --------------
// Event Handlers
// --------------

pub fn event_spawn_text_indicator(
    mut commands: Commands,
    mut ev_spawn_text_indicator: EventReader<SpawnTextIndicatorEvent>,
    font_assets: Res<FontAssets>,
) {
    for ev in ev_spawn_text_indicator.read() {
        let timer = Timer::from_seconds(2.0, TimerMode::Once);

        let text_indicator = TextIndicator { timer };

        commands.entity(ev.entity).with_children(|parent| {
            let text_style = TextStyle {
                font: font_assets.pixeloid_mono.clone(),
                font_size: 5.0,
                color: Color::WHITE,
            };
            let text_alignment = TextAlignment::Center;

            parent.spawn((
                Text2dBundle {
                    text: Text::from_section(ev.text.clone(), text_style.clone())
                        .with_alignment(text_alignment),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                },
                text_indicator,
            ));
        });
    }
}

// ------
// Plugin
// ------

pub struct TextIndicatorPlugin;

impl Plugin for TextIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_text_indicator, event_spawn_text_indicator))
            .add_event::<SpawnTextIndicatorEvent>();
    }
}
