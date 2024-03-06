use std::time::Duration;

use crate::CutsceneAssets;
use crate::GameState;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SplashscreenTimer(pub Timer);

pub struct SplashscreenPlugin;

impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameState::Splash),
            |mut commands: Commands, q_menu_components: Query<(Entity, &Splashscreen)>| {
                for (e, _) in q_menu_components.iter() {
                    commands.entity(e).despawn_recursive();
                }
            },
        )
        .add_systems(
            OnEnter(GameState::Splash),
            |mut splashscreen_timer: ResMut<SplashscreenTimer>| {
                splashscreen_timer.0 = Timer::new(Duration::from_secs(3), TimerMode::Once);
            },
        )
        .add_systems(Update, (switch_to_menu).run_if(in_state(GameState::Splash)))
        .add_systems(OnEnter(GameState::Splash), setup_splashscreen)
        .add_systems(OnExit(GameState::Splash), cleanup_splashscreen)
        .init_resource::<SplashscreenTimer>();
    }
}

#[derive(Component)]
struct Splashscreen;

fn setup_splashscreen(mut commands: Commands, cutscene_assets: Res<CutsceneAssets>) {
    info!("splashscreen");

    commands.spawn(Camera2dBundle::default());

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
            Splashscreen,
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
                UiImage::new(cutscene_assets.splash.clone()),
            ));
        });
}

fn switch_to_menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut splashscreen_timer: ResMut<SplashscreenTimer>,
    time: Res<Time>,
) {
    splashscreen_timer.0.tick(time.delta());

    if splashscreen_timer.0.just_finished() {
        next_state.set(GameState::Menu);
    }
}

fn cleanup_splashscreen(mut commands: Commands, menu: Query<Entity, With<Splashscreen>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
