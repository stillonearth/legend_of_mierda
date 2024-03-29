use bevy::prelude::*;



#[derive(Component)]
pub struct UIPlayerHealth;

#[derive(Component)]
pub struct UIGameOver;

#[derive(Component)]
pub struct UIGameplayWave;

#[derive(Component)]
pub struct UIHighscore;

#[derive(Component)]
struct UIGameOverButton;

#[derive(Component)]
pub struct UIGamePlay;

pub(crate) fn despawn_ui(mut commands: Commands, query: Query<Entity, With<UIGamePlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub(crate) fn draw_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // alextime face
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    bottom: Val::Px(0.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            UIGamePlay,
            Name::new("ui face"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(125.0),
                        height: Val::Px(125.0),
                        margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(asset_server.load("avatars/alextime.png")),
            ));
        });
    // health bar
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    bottom: Val::Px(35.0),
                    left: Val::Px(20.0),
                    padding: UiRect {
                        right: Val::Px(75.0),
                        ..default()
                    },
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            UIGamePlay,
            Name::new("ui healthbar"),
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    background_color: Color::RED.into(),
                    ..default()
                },))
                .insert(UIPlayerHealth);
        });
    // highscore
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    bottom: Val::Px(35.0),
                    right: Val::Px(10.0),
                    padding: UiRect {
                        right: Val::Px(75.0),
                        ..default()
                    },
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            UIGamePlay,
            Name::new("highscore"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "SCORE: 0",
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                UIHighscore,
            ));
        });

    // Wave
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                // visibility: Visibility::Hidden,
                ..default()
            },
            UIGamePlay,
            Name::new("Wave Text"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "ui wave text",
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                UIGameplayWave,
            ));
        });
}
