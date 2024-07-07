use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{ui::UIGameOver, AudioAssets, ButtonColors, ChangeState, FontAssets, GameState};

#[derive(Event, Clone)]
pub struct GameOverEvent;

#[derive(Event, Clone)]
pub struct GameWinEvent;

#[derive(Component)]
struct UIGameOverButton;

#[derive(Component)]
struct UIGameOverText;

pub fn event_game_over(
    mut ev_game_over: EventReader<GameOverEvent>,
    mut q_ui_game_over: Query<(&mut Visibility, &UIGameOver)>,
    mut next_state: ResMut<NextState<GameState>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut text_query: Query<(&mut Text, &UIGameOverText)>,
) {
    for _ in ev_game_over.read() {
        for (mut visibility, _) in q_ui_game_over.iter_mut() {
            *visibility = Visibility::Visible;
        }

        audio.play(audio_assets.gameover.clone()).with_volume(0.5);
        next_state.set(GameState::GameOver);
        for (mut text_component, _) in text_query.iter_mut() {
            text_component.sections[0].value = "  JUEGO\nTERMINADO".to_string();
        }
    }
}

pub fn event_game_won(
    mut ev_game_over: EventReader<GameWinEvent>,
    mut q_ui_game_over: Query<(&mut Visibility, &UIGameOver)>,
    mut next_state: ResMut<NextState<GameState>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut text_query: Query<(&mut Text, &UIGameOverText)>,
) {
    for _ in ev_game_over.read() {
        for (mut visibility, _) in q_ui_game_over.iter_mut() {
            *visibility = Visibility::Visible;
        }

        // audio.play(audio_assets.gameover.clone()).with_volume(0.5);
        next_state.set(GameState::GameOver);
        for (mut text_component, _) in text_query.iter_mut() {
            text_component.sections[0].value = "  JUEGO\nGANADO".to_string();
        }
    }
}

pub(crate) fn despawn_ui(mut commands: Commands, query: Query<Entity, With<UIGameOver>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub(crate) fn draw_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    print!("draw game over");

    // game over
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            UIGameOver,
            Name::new("ui game over"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "  JUEGO\nTERMINADO",
                    TextStyle {
                        font: font_assets.pixeloid_mono.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ),
                UIGameOverText,
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(318.0),
                            height: Val::Px(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(100.0),
                            ..Default::default()
                        },
                        background_color: Color::rgba_u8(0, 0, 0, 255).into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::GREEN,
                        hovered: Color::LIME_GREEN,
                    },
                    ChangeState(GameState::GamePlay),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "START OVER",
                            TextStyle {
                                font_size: 50.0,
                                font: font_assets.pixeloid_mono.clone(),
                                color: Color::WHITE,
                            },
                        ),
                        UIGameOverButton,
                    ));
                });
        });
}

fn click_gameover_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonColors, Option<&ChangeState>),
        (Changed<Interaction>, With<Button>),
    >,
    mut start_game_button: Query<(&mut Text, &UIGameOverButton)>,
) {
    for (interaction, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                }
            }
            Interaction::Hovered => {
                for (mut text, _) in start_game_button.iter_mut() {
                    text.sections[0].style.color = button_colors.hovered;
                }
            }
            Interaction::None => {
                for (mut text, _) in start_game_button.iter_mut() {
                    text.sections[0].style.color = button_colors.normal;
                }
            }
        }
    }
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (event_game_over, event_game_won).run_if(in_state(GameState::GamePlay)),
        )
        .add_systems(
            Update,
            click_gameover_button.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(OnEnter(GameState::GameOver), draw_ui)
        .add_systems(OnExit(GameState::GameOver), despawn_ui)
        .add_event::<GameWinEvent>()
        .add_event::<GameOverEvent>();
    }
}
