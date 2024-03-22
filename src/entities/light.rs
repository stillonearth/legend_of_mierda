use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_magic_light_2d::prelude::*;
use ffmpeg_next::util::range;

use crate::{
    entities::player::Player, load_texture_atlas, sprites::LANTERN_ASSET_SHEET, GameState,
};

#[derive(Component)]
pub struct LevelLight;

#[derive(Component)]
pub struct PlayerLight;

#[derive(Clone, PartialEq, Debug, Default, Component, Reflect)]
pub struct Lantern;

#[derive(Clone, Default, Bundle)]
pub struct LanternBundle {
    pub sprite_bundle: SpriteSheetBundle,
    pub lantern: Lantern,
}

pub fn create_lantern_bundle(
    asset_server: &AssetServer,
    texture_atlasses: &mut Assets<TextureAtlas>,
) -> LanternBundle {
    let atlas_handle = load_texture_atlas(
        LANTERN_ASSET_SHEET.to_string(),
        asset_server,
        1,
        1,
        None,
        32.,
        texture_atlasses,
    );

    let sprite_bundle = SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(0),
        ..default()
    };

    LanternBundle {
        sprite_bundle,
        lantern: Lantern,
    }
}

impl LdtkEntity for LanternBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlasses: &mut Assets<TextureAtlas>,
    ) -> LanternBundle {
        create_lantern_bundle(asset_server, texture_atlasses)
    }
}

pub fn setup_light(
    mut q_lanterns: Query<(&GlobalTransform, &Lantern)>,
    mut q_level_lights: Query<(Entity, &LevelLight)>,
    mut player_lights: Query<(Entity, &PlayerLight)>,
    mut commands: Commands,
    q_players: Query<(Entity, &GlobalTransform, &Player)>,
) {
    if q_level_lights.iter().count() == 0 {
        for (t, _) in q_lanterns.iter() {
            // we should update lantern positions elewhere in next frame, this is hack
            if t.translation() == Vec3::ZERO {
                return;
            }

            commands.spawn((
                OmniLightSource2D {
                    intensity: 0.6,
                    color: Color::rgb_u8(255, 125, 125),
                    falloff: Vec3::new(5.5, 10.0, 0.005),
                    ..default()
                },
                SpatialBundle {
                    transform: Transform {
                        translation: t.translation(),
                        ..default()
                    },
                    ..default()
                },
                LevelLight,
            ));
        }

        commands.spawn((
            SkylightLight2D {
                color: Color::rgb_u8(93, 158, 179),
                intensity: 0.015,
            },
            Name::new("global_skylight"),
        ));
    }

    if player_lights.iter().count() == 0 {
        if q_players.is_empty() {
            return;
        }

        let player_global_transform = q_players.iter().next().unwrap().1.clone();

        commands.spawn((
            OmniLightSource2D {
                intensity: 1.0,
                color: Color::rgb_u8(255, 255, 255),
                falloff: Vec3::new(1.5, 5.0, 0.05),
                ..default()
            },
            SpatialBundle {
                transform: Transform {
                    translation: player_global_transform.translation(),
                    ..default()
                },
                ..default()
            },
            PlayerLight,
        ));
    }
}

fn update_player_light_position(
    q_players: Query<(Entity, &GlobalTransform, &Player)>,
    mut q_player_lights: Query<(&mut Transform, &PlayerLight)>,
) {
    for (_, player_global_transform, _) in q_players.iter() {
        for (mut transform, _) in q_player_lights.iter_mut() {
            transform.translation = player_global_transform.translation();
        }
    }
}

// ------
// Plugin
// ------

pub struct LightPlugiin;

impl Plugin for LightPlugiin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<LanternBundle>("Lantern")
            .add_systems(
                Update,
                (setup_light, update_player_light_position).run_if(in_state(GameState::GamePlay)),
            );
    }
}
