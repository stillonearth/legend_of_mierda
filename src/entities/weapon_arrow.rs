use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};

use super::{
    mierda::{Mierda, MierdaHitEvent},
    pendejo::{Pendejo, PendejoHitEvent},
    player::Player,
};
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

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct WeaponArrowAttackEvent {}

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
    for (entity, _parent, _player_transform, _) in q_players.iter() {
        if q_arrows.p0().iter().count() == 0 {
            let timer_activation = WeaponArrowTimer(Timer::new(
                Duration::from_secs_f32(1.0),
                TimerMode::Repeating,
            ));

            let mut timer_hide = WeaponArrowHideTimer(Timer::new(
                Duration::from_secs_f32(0.2),
                TimerMode::Repeating,
            ));

            timer_hide.0.pause();

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    WeaponArrowBundle {
                        sprite_bundle: SpriteBundle {
                            texture: static_sprite_assets.arrow.clone(),
                            transform: Transform::from_translation(Vec3::new(20.0, 0., 0.)),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        weapon_arrow: WeaponArrow::Right,
                        timer_activation: timer_activation.clone(),
                        timer_deactivation: timer_hide.clone(),
                    },
                    Name::new("weapon arrow"),
                    ZIndex::Local(103),
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
                            transform: Transform::from_translation(Vec3::new(-20.0, 0., 0.)),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        weapon_arrow: WeaponArrow::Left,
                        timer_activation: timer_activation.clone(),
                        timer_deactivation: timer_hide.clone(),
                    },
                    Name::new("weapon arrow"),
                    ZIndex::Local(103),
                ));
            });
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
    mut ev_arrow_attack: EventWriter<WeaponArrowAttackEvent>,
    time: Res<Time>,
) {
    if queries.p0().iter().next().is_none() {
        return;
    }

    for (entity, mut transform, mut visibility, arrow, mut timer_activate, mut timer_hide) in
        queries.p1().iter_mut()
    {
        timer_activate.0.tick(time.delta());
        timer_hide.0.tick(time.delta());

        if timer_activate.0.just_finished() {
            *visibility = Visibility::Visible;

            timer_hide.0.unpause();

            let end = match arrow {
                WeaponArrow::Right => Vec3::new(55., 0., 0.),
                WeaponArrow::Left => Vec3::new(-55., 0., 0.),
            };

            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_secs_f32(0.15),
                TransformPositionLens {
                    start: match arrow {
                        WeaponArrow::Right => Vec3::new(20., 0., 0.),
                        WeaponArrow::Left => Vec3::new(-20., 0., 0.),
                    },
                    end,
                },
            );

            ev_arrow_attack.send(WeaponArrowAttackEvent {});
            commands.entity(entity).insert(Animator::new(tween));
        }

        if timer_hide.0.just_finished() {
            timer_hide.0.pause();
            *visibility = Visibility::Hidden;

            transform.translation = match arrow {
                WeaponArrow::Right => Vec3::new(20., 0., 0.),
                WeaponArrow::Left => Vec3::new(-20., 0., 0.),
            };
        }
    }
}

fn handle_arrow_attack(
    mut arrow_attack_events: EventReader<WeaponArrowAttackEvent>,
    mut ev_mierda_hit: EventWriter<MierdaHitEvent>,
    mut ev_pendejo_hit: EventWriter<PendejoHitEvent>,
    mut queries: ParamSet<(
        Query<(&Transform, &Player)>,
        Query<(Entity, &Transform, &Pendejo)>,
        Query<(Entity, &Transform, &Mierda)>,
    )>,
) {
    for _ in arrow_attack_events.read() {
        if queries.p0().iter().len() == 0 {
            return;
        }

        let player_translation = queries.p0().iter().next().unwrap().0.translation;

        for (e, transfrom, _) in queries.p1().iter() {
            let translation = transfrom.translation;

            if (translation.z - player_translation.z).abs() > 16.0 {
                continue;
            }

            let distance = translation.distance(player_translation).abs();

            if distance > 40.0 {
                continue;
            }

            ev_pendejo_hit.send(PendejoHitEvent(e));
        }

        for (e, transfrom, _) in queries.p2().iter() {
            let translation = transfrom.translation;

            if (translation.z - player_translation.z).abs() > 16.0 {
                continue;
            }

            let distance = translation.distance(player_translation).abs();

            if distance > 40.0 {
                continue;
            }

            ev_mierda_hit.send(MierdaHitEvent(e));
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
                (inject_arrow_sprite, animate_arrow, handle_arrow_attack)
                    .run_if(in_state(GameState::GamePlay)),
            )
            .add_event::<WeaponArrowAttackEvent>();
    }
}
