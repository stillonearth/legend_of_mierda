use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_particle_systems::Lerpable;


use crate::{
    controls::ControlEvent,
    entities::{
        player::Player,
    },
};
use crate::{GameState};

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
