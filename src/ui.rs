use bevy::prelude::*;

#[derive(Component)]
pub struct UiPlayerHealth;

#[derive(Component)]
pub struct UiGameOver;

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
                UiImage::new(asset_server.load("face.png")),
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
                .insert(UiPlayerHealth);
        });
    // game over
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    top: Val::Percent(33.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            UiGameOver,
            Name::new("ui game over"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME\nOVER",
                TextStyle {
                    font: asset_server.load("fonts/Mexicana.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ));
        });
}
