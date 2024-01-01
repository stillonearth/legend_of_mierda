

use bevy::prelude::*;

use bevy_particle_systems::*;





use crate::{
    components::Player,
    enemies::{Mierda, MierdaHitEvent},
    sprites::{AnimationDirection, CharacterAnimation},
    ui::{self, UIGameOver},
};

#[derive(Event, Clone)]
pub struct PlayerAttackEvent {
    pub entity: Entity,
}

#[derive(Event, Clone)]
pub struct PlayerHitEvent {
    pub entity: Entity,
}

#[derive(Event, Clone)]
pub struct GameOverEvent;

#[derive(Event, Clone)]
pub struct LevelChangeEvent {
    pub(crate) level_id: usize,
}

pub fn event_player_attack(
    mut ev_player_attack: EventReader<PlayerAttackEvent>,
    mut ev_mierda_hit: EventWriter<MierdaHitEvent>,
    mut q_player: Query<(Entity, &Transform, &CharacterAnimation), With<Player>>,
    mut los_mierdas: Query<(Entity, &Transform, &mut Mierda)>,
) {
    for ev in ev_player_attack.iter() {
        let (_, transform, char_animation) = q_player.get_mut(ev.entity).unwrap();

        let player_position = transform.translation;
        let player_orientation = char_animation.direction;

        // find all mierdas in range
        for (entity, mierda_transform, _) in los_mierdas.iter_mut().filter(|(_, _, m)| !m.is_dummy)
        {
            let mierda_position = mierda_transform.translation;

            let distance = player_position.distance(mierda_position);

            if distance >= 75. {
                continue;
            }

            // cause damage accrodign to player_orientation
            let is_merda_attacked = match player_orientation {
                AnimationDirection::Up => player_position.y < mierda_position.y,
                AnimationDirection::Down => player_position.y > mierda_position.y,
                AnimationDirection::Left => player_position.x > mierda_position.x,
                AnimationDirection::Right => player_position.x < mierda_position.x,
            };

            if !is_merda_attacked {
                continue;
            }

            ev_mierda_hit.send(MierdaHitEvent(entity));
        }
    }
}

pub fn event_player_hit(
    mut commands: Commands,
    mut ev_player_hit_reader: EventReader<PlayerHitEvent>,
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut q_player: Query<(Entity, &GlobalTransform, &mut Player)>,
    mut q_ui_healthbar: Query<(Entity, &mut Style, &ui::UIPlayerHealth)>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_player_hit_reader.iter() {
        let (_, player_transform, mut player) = q_player.get_mut(ev.entity).unwrap();

        commands.spawn((
            ParticleSystemBundle {
                transform: (*player_transform).into(),
                particle_system: ParticleSystem {
                    spawn_rate_per_second: 0.0.into(),
                    texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                    max_particles: 5_000,
                    initial_speed: (0.0..300.0).into(),
                    scale: 1.0.into(),
                    velocity_modifiers: vec![
                        VelocityModifier::Drag(0.001.into()),
                        VelocityModifier::Vector(Vec3::new(0.0, -400.0, 0.0).into()),
                    ],
                    color: (Color::RED..Color::rgba(1.0, 0.0, 0.0, 0.0)).into(),
                    bursts: vec![ParticleBurst {
                        time: 0.0,
                        count: 1000,
                    }],
                    looping: false,
                    ..ParticleSystem::default()
                },
                ..default()
            },
            Playing,
        ));

        if player.health < 10 {
            ev_game_over.send(GameOverEvent);
            continue;
        } else {
            player.health -= 10;

            for (_, mut style, _) in q_ui_healthbar.iter_mut() {
                style.width = Val::Percent(player.health as f32);
            }
        }
    }
}

pub fn event_game_over(
    mut ev_game_over: EventReader<GameOverEvent>,
    mut q_ui_game_over: Query<(&mut Visibility, &UIGameOver)>,
) {
    for _ in ev_game_over.iter() {
        for (mut visibility, _) in q_ui_game_over.iter_mut() {
            *visibility = Visibility::Visible;
        }
    }
}
