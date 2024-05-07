use std::time::Duration;

use bevy::prelude::*;

use crate::entities::{
    player::Player,
};
use crate::{loading::StaticSpriteAssets, GameState};

// ----------
// Components
// ----------

#[derive(Component, Clone, Copy, Default)]
pub struct WeaponSpeargun;

// ------
// Events
// ------

#[derive(Event, Clone)]
pub struct SpeargunShootEvent {}

// -------
// Bundles
// -------

#[derive(Clone, Default, Bundle)]
pub struct WeaponSpeargunBundle {
    pub sprite_bundle: SpriteBundle,
    pub speargun: WeaponSpeargun,
    pub timer_activation: WeaponSpeargunTimer,
    pub timer_deactivation: WeaponSpeargunHideTimer,
}

// ---------
// Resources
// ---------

#[derive(Resource, Default, Clone, Component)]
pub struct WeaponSpeargunTimer(pub Timer);

#[derive(Resource, Default, Clone, Component)]
pub struct WeaponSpeargunHideTimer(pub Timer);

fn inject_speargun_sprite(
    mut commands: Commands,
    q_players: Query<(Entity, &Parent, &Transform, &Player)>,
    mut q_spearguns: ParamSet<(Query<(&mut Transform, &WeaponSpeargun), Without<Player>>,)>,
    static_sprite_assets: Res<StaticSpriteAssets>,
) {
    for (entity, _parent, _player_transform, _) in q_players.iter() {
        if q_spearguns.p0().iter().count() == 0 {
            let timer_activation = WeaponSpeargunTimer(Timer::new(
                Duration::from_secs_f32(1.0),
                TimerMode::Repeating,
            ));

            let mut timer_hide = WeaponSpeargunHideTimer(Timer::new(
                Duration::from_secs_f32(0.2),
                TimerMode::Repeating,
            ));

            timer_hide.0.pause();

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    WeaponSpeargunBundle {
                        sprite_bundle: SpriteBundle {
                            texture: static_sprite_assets.speargun.clone(),
                            transform: Transform::from_translation(Vec3::new(20.0, 0., 0.)),
                            // visibility: Visibility::Hidden,
                            ..default()
                        },
                        speargun: WeaponSpeargun,
                        timer_activation: timer_activation.clone(),
                        timer_deactivation: timer_hide.clone(),
                    },
                    Name::new("weapon speargun"),
                    ZIndex::Local(103),
                ));
            });
        }
    }
}

fn control(
    input: Res<Input<KeyCode>>,
    mut q_speargun: Query<(&mut Transform, &WeaponSpeargun), Without<Player>>,
) {
    for (mut transform, _) in q_speargun.iter_mut() {
        let mut angle = 0.00;
        if input.pressed(KeyCode::Left) {
            angle = 0.1;
        }
        if input.pressed(KeyCode::Right) {
            angle = -0.1;
        }

        transform.rotation *= Quat::from_rotation_z(angle);
    }
}

// fn adjust_speargun_angle(
//     q_windows: Query<&Window, With<PrimaryWindow>>,
//     q_players: Query<(Entity, &Parent, &Transform, &Player)>,
//     mut paramset: ParamSet<(
//         Query<(&mut Transform, &WeaponSpeargun), Without<Player>>,
//         Query<(&mut Transform, &SpriteCamera), Without<Player>>,
//         Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
//     )>,
//     level_selection: Res<LevelSelection>,
//     projects: Query<&Handle<LdtkProject>>,
//     project_assets: Res<Assets<LdtkProject>>,
// ) {
//     let binding1 = paramset.p1();
//     let (camera_transform, _) = binding1.single();
//     let camera_position = Vec2::new(
//         camera_transform.translation.x,
//         camera_transform.translation.y,
//     );
//     let project = project_assets.get(projects.single());
//     if project.is_none() {
//         return;
//     }
//     let project = project.unwrap();

//     let mut level_position: Vec2 = Vec2::ZERO;
//     let mut level_size: Vec2 = Vec2::ZERO;

//     for (level_transform, level_iid) in paramset.p2().iter() {
//         if let Some(ldtk_level) = project.get_raw_level_by_iid(level_iid.get()) {
//             let level = &ldtk_level;
//             if level_selection.is_match(
//                 &LevelIndices {
//                     level: 0,
//                     ..default()
//                 },
//                 level,
//             ) {
//                 level_position =
//                     Vec2::new(level_transform.translation.x, level_transform.translation.y);
//                 level_size = Vec2::new(level.px_wid as f32, level.px_hei as f32);
//             }
//         }
//     }

//     if let Some(cursor_position) = q_windows.single().cursor_position() {
//         if q_players.iter().next().is_none() {
//             return;
//         }

//         let player_transform = q_players.iter().next().unwrap().2;
//         let player_position = player_transform.translation;
//         let player_position = Vec2::new(player_position.x, player_position.y);

//         println!("Player Position: {:?}", player_position);
//         println!("Cursor Position: {:?}", cursor_position);
//         println!("Camera Position: {:?}", camera_position);
//         println!("Level Position: {:?}", level_position);
//         println!(
//             "Level-Camera Position: {:?}",
//             level_position - camera_position
//         );
//         println!(
//             "Player Position-Level Size: {:?}",
//             player_position - level_size
//         );

//         let angle = {
//             let direction = cursor_position - level_size;
//             direction.angle_between(Vec2::new(1.0, 0.0))
//         };

//         println!("Angle: {}", angle);

//         // rotate the speargun
//         for (mut transform, _) in paramset.p0().iter_mut() {
//             transform.rotation = Quat::from_rotation_z(angle);
//         }
//     }
// }

// fn animate_arrow(
//     mut commands: Commands,
//     mut queries: ParamSet<(
//         Query<(&Parent, &Transform, &Player)>,
//         Query<(
//             Entity,
//             &mut Transform,
//             &mut Visibility,
//             &WeaponSpeargun,
//             &mut WeaponSpeargunTimer,
//             &mut WeaponSpeargunHideTimer,
//         )>,
//     )>,
//     mut ev_arrow_attack: EventWriter<WeaponSpeargunAttackEvent>,
//     time: Res<Time>,
// ) {
//     if queries.p0().iter().next().is_none() {
//         return;
//     }

//     for (entity, mut transform, mut visibility, arrow, mut timer_activate, mut timer_hide) in
//         queries.p1().iter_mut()
//     {
//         timer_activate.0.tick(time.delta());
//         timer_hide.0.tick(time.delta());

//         if timer_activate.0.just_finished() {
//             *visibility = Visibility::Visible;

//             timer_hide.0.unpause();

//             let end = match arrow {
//                 WeaponSpeargun::Right => Vec3::new(55., 0., 0.),
//                 WeaponSpeargun::Left => Vec3::new(-55., 0., 0.),
//             };

//             let tween = Tween::new(
//                 EaseFunction::QuadraticInOut,
//                 Duration::from_secs_f32(0.15),
//                 TransformPositionLens {
//                     start: match arrow {
//                         WeaponSpeargun::Right => Vec3::new(20., 0., 0.),
//                         WeaponSpeargun::Left => Vec3::new(-20., 0., 0.),
//                     },
//                     end,
//                 },
//             );

//             ev_arrow_attack.send(WeaponSpeargunAttackEvent {});
//             commands.entity(entity).insert(Animator::new(tween));
//         }

//         if timer_hide.0.just_finished() {
// println!("Cursor is inside the primary window, at {:?}", position);
//             timer_hide.0.pause();
//             *visibility = Visibility::Hidden;

//             transform.translation = match arrow {
//                 WeaponSpeargun::Right => Vec3::new(20., 0., 0.),
//                 WeaponSpeargun::Left => Vec3::new(-20., 0., 0.),
//             };
//         }
//     }
// }

// fn handle_arrow_attack(
//     mut arrow_attack_events: EventReader<WeaponSpeargunAttackEvent>,
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

//             ev_enemy_hit.send(EnemyHitEvent(e));
//         }
//     }
// }

// ------
// Plugin
// ------

pub struct WeaponSpeargunPlugin;

impl Plugin for WeaponSpeargunPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponSpeargunTimer>()
            // Event Handlers
            .add_systems(
                Update,
                (inject_speargun_sprite, control).run_if(in_state(GameState::GamePlay)),
            )
            .add_event::<SpeargunShootEvent>();
    }
}
