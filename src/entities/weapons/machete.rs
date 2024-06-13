use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_particle_systems::Lerpable;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};

use crate::{
    controls::ControlEvent,
    entities::{
        characters::enemy::{Enemy, EnemyHitEvent},
        player::Player,
    },
};
use crate::{loading::StaticSpriteAssets, GameState};

// ----------
// Components
// ----------

#[derive(Component, Clone, Copy, Default)]
pub struct Machete {}

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct MacheteIndictorBundle {
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub machete_indicator: Machete,
    pub timer_activation: MacheteTimer,
}

// ---------
// Resources
// ---------

#[derive(Resource, Default, Clone, Component)]
pub struct MacheteTimer(pub Timer);

// -------
// Systems
// -------

fn inject_machete_indicator(
    mut commands: Commands,
    q_players: Query<(Entity, &Parent, &Transform, &Player)>,
    mut q_machate_indicator: ParamSet<(Query<(&mut Transform, &Machete), Without<Player>>,)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, _parent, _player_transform, _) in q_players.iter() {
        if q_machate_indicator.p0().iter().count() == 0 {
            let machete_timer = MacheteTimer(Timer::new(
                Duration::from_secs_f32(1.0),
                TimerMode::Repeating,
            ));

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    MacheteIndictorBundle {
                        machete_indicator: Machete {},
                        timer_activation: machete_timer.clone(),
                        material_mesh_2d_bundle: MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(80.).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::PURPLE.with_a(0.5))),
                            ..default()
                        },
                    },
                    Name::new("machete radius indicator"),
                    ZIndex::Local(103),
                ));
            });
        }
    }
}

pub fn handle_machete_attack(
    time: Res<Time>,
    mut q_machete: Query<(Entity, &Transform, &mut MacheteTimer), With<Machete>>,
    mut ev_control: EventWriter<ControlEvent>,
) {
    for (_, _, mut machete_timer) in q_machete.iter_mut() {
        machete_timer.0.tick(time.delta());

        if machete_timer.0.just_finished() {
            ev_control.send(ControlEvent {
                attack: true,
                ..Default::default()
            });
        }
    }
}

fn animate_machete_indicator(
    mut q_machete: Query<(Entity, &mut Handle<ColorMaterial>, &mut MacheteTimer), With<Machete>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (_, mut material, timer) in q_machete.iter_mut() {
        let current_value = timer.0.percent_left().max(0.2);
        let percentage = timer.0.percent_left().max(0.2).lerp(0.8, current_value);
        *material = materials.add(ColorMaterial::from(Color::PURPLE.with_a(percentage)));
    }
}

// fn animate_arrow(
//     mut commands: Commands,
//     mut queries: ParamSet<(
//         Query<(&Parent, &Transform, &Player)>,
//         Query<(
//             Entity,
//             &mut Transform,
//             &mut Visibility,
//             &WeaponArrow,
//             &mut MacheteIndictorTimer,
//             &mut WeaponArrowHideTimer,
//         )>,
//     )>,
//     mut ev_arrow_attack: EventWriter<WeaponArrowAttackEvent>,
//     time: Res<Time>,
// ) {
//     if queries.p0().iter().next().is_none() {
//         return;
//     }animate_machete_indicator;

//         if timer_activate.0.just_finished() {
//             *visibility = Visibility::Visible;

//             timer_hide.0.unpause();

//             let end = match arrow {
//                 WeaponArrow::Right => Vec3::new(55., 0., 0.),
//                 WeaponArrow::Left => Vec3::new(-55., 0., 0.),
//             };

//             let tween = Tween::new(
//                 EaseFunction::QuadraticInOut,
//                 Duration::from_secs_f32(0.15),
//                 TransformPositionLens {
//                     start: match arrow {
//                         WeaponArrow::Right => Vec3::new(20., 0., 0.),
//                         WeaponArrow::Left => Vec3::new(-20., 0., 0.),
//                     },
//                     end,
//                 },
//             );

//             ev_arrow_attack.send(WeaponArrowAttackEvent {});
//             commands.entity(entity).insert(Animator::new(tween));
//         }

//         if timer_hide.0.just_finished() {
//             timer_hide.0.pause();
//             *visibility = Visibility::Hidden;

//             transform.translation = match arrow {
//                 WeaponArrow::Right => Vec3::new(20., 0., 0.),
//                 WeaponArrow::Left => Vec3::new(-20., 0., 0.),
//             };
//         }
//     }
// }

// fn handle_arrow_attack(
//     mut arrow_attack_events: EventReader<WeaponArrowAttackEvent>,
//     mut ev_enemy_hit: EventWriter<EnemyHitEvent>,
//     mut queries: ParamSet<(
//         Query<(&Transform, &Player)>,
//         Query<(Entity, &Transform, &Enemy)>,
//     )>,
// ) {
//     for _ in arrow_attack_events.read() {
//         if queries.p0().iter().len() == 0 {
//             return;
//         }

//         let player_translation = queries.p0().iter().next().unwrap().0.translation;

//         for (e, transfrom, _) in queries.p1().iter() {
//             let translation = transfrom.translation;

//             if (translation.z - player_translation.z).abs() > 16.0 {
//                 continue;
//             }

//             let distance = translation.distance(player_translation).abs();

//             if distance > 40.0 {
//                 continue;
//             }

//             ev_enemy_hit.send(EnemyHitEvent {
//                 entity: e,
//                 damage: 88,
//             });
//         }
//     }
// }

// ------
// Plugin
// ------

pub struct MachetePlugin;

impl Plugin for MachetePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MacheteTimer>()
            // Event Handlers
            .add_systems(
                Update,
                (
                    inject_machete_indicator,
                    handle_machete_attack,
                    animate_machete_indicator,
                )
                    .run_if(in_state(GameState::GamePlay)),
            );
        // .add_event::<WeaponArrowAttackEvent>();
    }
}
