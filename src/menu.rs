use crate::loading::FontAssets;
use crate::loading::TextureAssets;
use crate::CutsceneAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Menu),
            |mut commands: Commands, q_menu_components: Query<(Entity, &Menu)>| {
                for (e, _) in q_menu_components.iter() {
                    commands.entity(e).despawn_recursive();
                }
            },
        )
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::WHITE,
            hovered: Color::BLACK,
        }
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct StartGameButton;

fn setup_menu(
    mut commands: Commands,
    cutscene_assets: Res<CutsceneAssets>,
    font_assets: Res<FontAssets>,
) {
    info!("menu");

    commands.spawn((Camera2dBundle::default()));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
            Name::new("cutscene image container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(cutscene_assets.main_menu.clone()),
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::right(Val::Percent(20.0)),
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(318.0),
                            height: Val::Px(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: Color::rgba_u8(0, 0, 0, 0).into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Gameplay),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "START",
                            TextStyle {
                                font_size: 100.0,
                                font: font_assets.pixeloid_mono.clone(),
                                color: Color::WHITE.into(),
                                ..default()
                            },
                        ),
                        StartGameButton,
                    ));
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonColors, Option<&ChangeState>),
        (Changed<Interaction>, With<Button>),
    >,
    mut start_game_button: Query<(&mut Text, &StartGameButton)>,
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
                    text.sections[0].style.color = button_colors.hovered.clone().into();
                }
            }
            Interaction::None => {
                for (mut text, _) in start_game_button.iter_mut() {
                    text.sections[0].style.color = button_colors.normal.clone().into();
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
