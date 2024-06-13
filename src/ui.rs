use bevy::prelude::*;

#[derive(Component)]
pub struct UIPlayerHealth;

#[derive(Component)]
pub struct UIGameOver;

#[derive(Component)]
pub struct UIGameplayWave;

#[derive(Component)]
pub struct UIWeaponName;

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

    // Weapon - Gun
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    bottom: Val::Px(25.0),
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
            Name::new("Weapon gun image"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(320.0),
                        height: Val::Px(45.0),
                        // margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(asset_server.load("sprites/speargun.png")),
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    bottom: Val::Px(15.0),
                    right: Val::Px(20.0),
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
            Name::new("Weapon name"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "SPEARGUN",
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                UIWeaponName,
            ));
        });

    // Weapon - Machete
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    bottom: Val::Px(100.0),
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
            Name::new("Weapon machete  image"),
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(320.0),
                        height: Val::Px(45.0),
                        // margin: UiRect::top(Val::VMin(5.)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UiImage::new(asset_server.load("sprites/machete.png")),
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexStart,
                    bottom: Val::Px(85.0),
                    right: Val::Px(20.0),
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
            Name::new("Weapon name"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "MACHETE",
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidMono-d94EV.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                UIWeaponName,
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

    // Highscore
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::FlexEnd,
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
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
}
