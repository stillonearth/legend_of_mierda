//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::prelude::*;
use bevy_particle_systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ParticleSystemPlugin) // prevents blurry sprites
        .add_systems(Startup, spawn_particles)
        .run();
}

fn spawn_particles(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 50_000,
                texture: ParticleTexture::Sprite(asset_server.load("px.png")),
                spawn_rate_per_second: 1000.0.into(),
                initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
                lifetime: JitteredValue::jittered(8.0, -2.0..2.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::PURPLE, 0.0),
                    CurvePoint::new(Color::RED, 0.5),
                    CurvePoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
                ])),
                looping: true,
                system_duration_seconds: 10.0,
                max_distance: Some(300.0),
                scale: 2.0.into(),
                bursts: vec![
                    ParticleBurst::new(0.0, 1000),
                    ParticleBurst::new(2.0, 1000),
                    ParticleBurst::new(4.0, 1000),
                    ParticleBurst::new(6.0, 1000),
                    ParticleBurst::new(8.0, 1000),
                ],
                ..ParticleSystem::default()
            },
            ..ParticleSystemBundle::default()
        })
        .insert(Playing);
}
