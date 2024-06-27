use std::time::Duration;

use crate::loading::{AvatarAssets, CutsceneAssets, FontAssets};
use crate::GameState;
use bevy::prelude::*;

pub struct CutscenePlugin;

#[derive(Resource)]
struct CutsceneState {
    timer: Timer,
    timer_count: usize,
}

#[derive(Component)]
struct Cutscene;

#[derive(Component)]
struct CutsceneAvatarAlextime;

#[derive(Component)]
struct CutsceneAvatarGennadiy;

#[derive(Component)]
struct CutsceneDialogText;

#[derive(Component)]
struct CutsceneTitleText;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Cutscene),
            |mut commands: Commands, q_menu_components: Query<(Entity, &Cutscene)>| {
                for (e, _) in q_menu_components.iter() {
                    commands.entity(e).despawn_recursive();
                }
            },
        )
        .add_systems(OnEnter(GameState::Cutscene), setup_cutscene)
        .add_systems(OnExit(GameState::Cutscene), cleanup_cutscene)
        .add_systems(
            Update,
            (handle_cutscene_text, handle_cutscene_termination)
                .run_if(in_state(GameState::Cutscene)),
        )
        .insert_resource(CutsceneState {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
            timer_count: 0,
        });
    }
}

fn setup_cutscene(
    mut commands: Commands,
    avatar_assets: Res<AvatarAssets>,
    cutscene_assets: Res<CutsceneAssets>,
    font_assets: Res<FontAssets>,
) {
    info!("cutscene");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            Cutscene,
            Name::new("cutscene dialog container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(320.0),
                        height: Val::Px(320.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(20.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(100),
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(avatar_assets.alextime.clone()),
                Name::new("avatar alextime"),
                CutsceneAvatarAlextime,
                // ZIndex::Local(100),
            ));

            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(320.0),
                        height: Val::Px(320.0),
                        position_type: PositionType::Absolute,
                        right: Val::Px(20.0),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    z_index: ZIndex::Global(100),
                    ..default()
                },
                UiImage::new(avatar_assets.gennadiy.clone()),
                Name::new("avatar gennadiy"),
                CutsceneAvatarGennadiy,
            ));

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            margin: UiRect::bottom(Val::Percent(10.)),
                            ..default()
                        },
                        z_index: ZIndex::Global(101),
                        ..default()
                    },
                    Name::new("dialog text"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "cutscene dialog text",
                            TextStyle {
                                font: font_assets.pixeloid_mono.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        CutsceneDialogText,
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    top: Val::Percent(10.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Global(100),
                ..default()
            },
            CutsceneDialogText,
            Cutscene,
            Name::new("cutscene title container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "   PRISON CPS 17\nMEXICO, BUENAVISTA",
                    TextStyle {
                        font: font_assets.pixeloid_mono.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ),
                CutsceneTitleText,
            ));
        });

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
            Cutscene,
            Name::new("cutscene image container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(512.0),
                        height: Val::Px(512.0),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(cutscene_assets.phone_call_1.clone()),
            ));
        });
}

fn cleanup_cutscene(mut commands: Commands, menu: Query<Entity, With<Cutscene>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_cutscene_termination(
    mut next_state: ResMut<NextState<GameState>>,
    cutscene_state: ResMut<CutsceneState>,
) {
    let cutscene_text = get_cutscene_dialog_text();
    if cutscene_text.len() <= cutscene_state.timer_count {
        next_state.set(GameState::GamePlay);
    }
}

fn handle_cutscene_text(
    time: Res<Time>,
    mut cutscene_state: ResMut<CutsceneState>,
    mut query: Query<(&mut Text, &CutsceneDialogText)>,
    mut avatar_set: ParamSet<(
        Query<(&mut Visibility, &CutsceneAvatarAlextime)>,
        Query<(&mut Visibility, &CutsceneAvatarGennadiy)>,
    )>,
) {
    cutscene_state.timer.tick(time.delta());

    if cutscene_state.timer.just_finished() {
        cutscene_state.timer_count += 1;
    }

    let cutscene_text = get_cutscene_dialog_text();
    if cutscene_text.len() <= cutscene_state.timer_count {
        return;
    }

    let (index, text) = cutscene_text[cutscene_state.timer_count].clone();

    for (mut visibility, _) in avatar_set.p0().iter_mut() {
        *visibility = match index {
            0 => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
    for (mut visibility, _) in avatar_set.p1().iter_mut() {
        *visibility = match index {
            1 => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }

    for (mut text_component, _) in query.iter_mut() {
        text_component.sections[0].value = text.clone();
    }
}

fn get_cutscene_dialog_text() -> Vec<(usize, String)> {
    vec![
        (
            1,
            "Your Highness, they've charged you with smuggling red caviar into the mess hall."
                .to_string(),
        ),
        (
            0,
            "Ah, Gena, they simply cannot resist my gourmet diplomacy".to_string(),
        ),
        (
            1,
            "Diplomacy, sire? It's more like high-seas gastronomy.".to_string(),
        ),
        (
            0,
            "Fear not, Gena. If they lock me up, I'll become the Cell Block Caviar Tsar!"
                .to_string(),
        ),
        (
            1,
            "Your platform: From the Elecrostahl to the cell, promising red caviar for all!"
                .to_string(),
        ),
    ]
}
