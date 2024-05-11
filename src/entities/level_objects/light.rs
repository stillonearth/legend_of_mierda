use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_magic_light_2d::prelude::*;

use crate::{
    entities::player::Player, ldtk::Wall, load_texture_atlas, sprites::LANTERN_ASSET_SHEET,
    GameState,
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
        _entity_instance: &EntityInstance,
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
    mut commands: Commands,
    _player_lights: Query<(Entity, &PlayerLight)>,
    q_lanterns: Query<(&GlobalTransform, &Lantern)>,
    q_level_lights: Query<(Entity, &LevelLight)>,
    q_walls: Query<(&GridCoords, &Wall)>,
    _q_players: Query<(Entity, &GlobalTransform, &Player)>,
    q_occluders: Query<(Entity, &LightOccluder2D)>,
) {
    if q_level_lights.iter().count() == 0 {
        let mut lights = vec![];

        for (t, _) in q_lanterns.iter() {
            // we should update lantern positions elewhere in next frame, this is hack
            if t.translation() == Vec3::ZERO {
                return;
            }

            let light_entity = commands
                .spawn((
                    OmniLightSource2D {
                        intensity: 0.01,
                        color: Color::rgb_u8(255, 125, 125),
                        falloff: Vec3::new(5.5, 10.0, 0.005),
                        jitter_intensity: 0.1,
                        jitter_translation: 3.0,
                    },
                    SpatialBundle {
                        transform: Transform {
                            translation: t.translation(),
                            ..default()
                        },
                        ..default()
                    },
                    LevelLight,
                ))
                .id();

            lights.push(light_entity);
        }

        commands
            .spawn(SpatialBundle::default())
            .insert(Name::new("lights"))
            .push_children(&lights);

        commands.spawn((
            SkylightLight2D {
                color: Color::rgb_u8(249, 143, 33),
                // intensity: 0.003,
                intensity: 0.03,
            },
            Name::new("global_skylight"),
        ));
    }

    if q_occluders.iter().count() == 0 {
        let mut occluders = vec![];

        for (wall_coord, _) in q_walls.iter() {
            // for _ in 0..1 {
            let occluder = LightOccluder2D {
                h_size: Vec2::splat(32.0),
            };
            let mask = SkylightMask2D {
                h_size: Vec2::splat(32.0),
            };
            let spatial_bundle = SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(wall_coord.x as f32, wall_coord.y as f32, 0.0),
                    // translation: Vec3::new(470., -120., 0.0),
                    ..default()
                },
                ..default()
            };

            let occluder_entity = commands
                .spawn((occluder, spatial_bundle, mask, Name::new("Occluder")))
                .id();

            occluders.push(occluder_entity);
        }

        commands
            .spawn(SpatialBundle::default())
            .insert(Name::new("occluders"))
            .push_children(&occluders);
    }

    // if player_lights.iter().count() == 0 {
    //     if q_players.is_empty() {
    //         return;
    //     }

    //     let player_global_transform = *q_players.iter().next().unwrap().1;

    //     commands.spawn((
    //         OmniLightSource2D {
    //             intensity: 1.0,
    //             color: Color::rgb_u8(255, 255, 255),
    //             falloff: Vec3::new(1.5, 5.0, 0.05),
    //             ..default()
    //         },
    //         SpatialBundle {
    //             transform: Transform {
    //                 translation: player_global_transform.translation(),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         PlayerLight,
    //     ));
    // }
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

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<LanternBundle>("Lantern")
            .add_systems(
                Update,
                (setup_light, update_player_light_position).run_if(in_state(GameState::GamePlay)),
            );
    }
}
