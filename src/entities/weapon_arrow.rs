use std::time::Duration;

use bevy::{prelude::*, transform::commands};
use bevy_tweening::{
    lens::TransformPositionLens, Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};

use super::player::Player;
use crate::{loading::StaticSpriteAssets, GameState};

// ----------
// Components
// ----------

#[derive(Component, Clone, Copy, Default)]
pub enum WeaponArrow {
    #[default]
    Right,
    Left,
}

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct WeaponArrowBundle {
    pub sprite_bundle: SpriteBundle,
    pub weapon_arrow: WeaponArrow,
    pub timer_activation: WeaponArrowTimer,
    pub timer_deactivation: WeaponArrowHideTimer,
}

// ---------
// Resources
// ---------

#[derive(Resource, Default, Clone, Component)]
pub struct WeaponArrowTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct WeaponArrowHideTimer(pub Timer);

fn inject_arrow_sprite(
    mut commands: Commands,
    q_players: Query<(Entity, &Parent, &Transform, &Player)>,
    mut q_arrows: ParamSet<(Query<(&mut Transform, &WeaponArrow), Without<Player>>,)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for (entity, parent, player_transform, _) in q_players.iter() {
        if q_arrows.p0().iter().count() == 0 {
            let timer_activation = WeaponArrowTimer(Timer::new(
                Duration::from_secs_f32(0.25),
                TimerMode::Repeating,
            ));

            let timer_deactivation = WeaponArrowHideTimer(Timer::new(
                Duration::from_secs_f32(0.5),
                TimerMode::Repeating,
            ));

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    WeaponArrowBundle {
                        sprite_bundle: SpriteBundle {
                            texture: static_sprite_assets.arrow.clone(),
                            // transform: Transform::from_translation(
                            //     player_transform.translation + Vec3::new(10.0, 0.0, 0.0),
                            // )
                            // .with_scale(Vec3::ONE * 0.5),
                            ..default()
                        },
                        weapon_arrow: WeaponArrow::Right,
                        timer_activation: timer_activation.clone(),
                        timer_deactivation: timer_deactivation.clone(),
                    },
                    Name::new("weapon arrow"),
                    ZIndex::Local(101),
                ));
            });

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    WeaponArrowBundle {
                        sprite_bundle: SpriteBundle {
                            sprite: Sprite {
                                flip_x: true,
                                ..default()
                            },
                            texture: static_sprite_assets.arrow.clone(),
                            // transform: Transform::from_translation(
                            //     player_transform.translation + Vec3::new(-10.0, 0.0, 0.0),
                            // )
                            // .with_scale(Vec3::ONE * 0.5),
                            ..default()
                        },
                        weapon_arrow: WeaponArrow::Left,
                        timer_activation: timer_activation.clone(),
                        timer_deactivation: timer_deactivation.clone(),
                    },
                    Name::new("weapon arrow"),
                    ZIndex::Local(101),
                ));
            });
        } else {
            // for (mut biboran_transform, _) in q_arrows.p1().iter_mut() {
            //     biboran_transform.translation =
            //         player_transform.translation + Vec3::new(25.0, 0.0, 0.0);
            // }

            // for (mut biboran_transform, _) in q_arrows.p2().iter_mut() {
            //     biboran_transform.translation =
            //         player_transform.translation + Vec3::new(-25.0, 0.0, 0.0);
            // }
        }
    }
}

fn animate_arrow(
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(&Parent, &Transform, &Player)>,
        Query<(
            Entity,
            &mut Transform,
            &mut Visibility,
            &WeaponArrow,
            &mut WeaponArrowTimer,
            &mut WeaponArrowHideTimer,
        )>,
    )>,
    time: Res<Time>,
) {
    if queries.p0().iter().next().is_none() {
        return;
    }

    for (entity, mut _transform, mut visibility, arrow, mut activation_timer, mut hide_timer) in
        queries.p1().iter_mut()
    {
        activation_timer.0.tick(time.delta());
        hide_timer.0.tick(time.delta());

        if activation_timer.0.just_finished() {
            *visibility = Visibility::Visible;

            let end = match arrow {
                WeaponArrow::Right => Vec3::new(55., 0., 0.),
                WeaponArrow::Left => Vec3::new(-55., 0., 0.),
            };

            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f32(0.25),
                TransformPositionLens {
                    start: Vec3::ZERO,
                    end: end,
                },
            )
            .with_repeat_count(RepeatCount::Infinite);

            commands.entity(entity).insert(Animator::new(tween));
        }

        if hide_timer.0.just_finished() {
            *visibility = Visibility::Hidden;
        }
    }
}

// ------
// Plugin
// ------

pub struct WeaponArrowPlugin;

impl Plugin for WeaponArrowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponArrowTimer>()
            // Event Handlers
            .add_systems(
                Update,
                (inject_arrow_sprite, animate_arrow).run_if(in_state(GameState::GamePlay)),
            );
    }
}
