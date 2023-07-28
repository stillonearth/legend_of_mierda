use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::components::*;

const ASPECT_RATIO: f32 = 16. / 9.;

pub fn update_level_selection(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for (level_handle, level_transform) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level_bounds = Rect {
                min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
                max: Vec2::new(
                    level_transform.translation.x + ldtk_level.level.px_wid as f32,
                    level_transform.translation.y + ldtk_level.level.px_hei as f32,
                ),
            };

            for player_transform in &player_query {
                // println!("Level bounds: {:?}", level_bounds);

                let player_within_x_bounds = player_transform.translation().x < level_bounds.max.x
                    && player_transform.translation().x > level_bounds.min.x;

                let player_within_y_bounds = player_transform.translation().y < level_bounds.max.y
                    && player_transform.translation().y > level_bounds.min.y;

                // let is_not_current_level = !level_selection.is_match(&0, &ldtk_level.level);

                if player_within_x_bounds && player_within_y_bounds {
                    // println!("\t\t\tswitch!");
                    *level_selection = LevelSelection::Iid(ldtk_level.level.iid.clone());
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&GlobalTransform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_translation = player_query.single().translation();

    let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

    // println!("number of levels: {}", level_query.iter().count());
    // return;

    for (level_transform, level_handle) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level = &ldtk_level.level;
            if level_selection.is_match(&0, level) {
                // println!("level: {}", level.iid);
                let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;
                orthographic_projection.viewport_origin = Vec2::ZERO;
                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    let height = (level.px_hei as f32 / 9.).round() * 9.;
                    let width = height * ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    let width = (level.px_wid as f32 / 16.).round() * 16.;
                    let height = width / ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.y =
                        (player_translation.y - level_transform.translation.y - height / 2.)
                            .clamp(0., level.px_hei as f32 - height);
                    camera_transform.translation.x = 0.;
                }

                camera_transform.translation.x += level_transform.translation.x;
                camera_transform.translation.y += level_transform.translation.y;
            }
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &CharacterAnimation,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
) {
    for (character_animation, mut timer, mut sprite, mut transform) in &mut query {
        timer.tick(time.delta());

        // For some reason sprite scaling is not working when set in the bundle
        transform.scale = Vec3::new(0.5, 0.5, 1.0);

        let indices = get_animation_indices(
            character_animation.animation_type,
            character_animation.direction,
        );

        // println!("indices: {:?}", indices);

        if timer.just_finished() {
            sprite.index = if (sprite.index >= indices.last) || (sprite.index < indices.first) {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn controls(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Velocity, &mut CharacterAnimation), With<Player>>,
    spritesheets: Res<PlayerSpritesheets>,
    // asset_server: Res<AssetServer>,
    // texture_atlases: &mut Assets<TextureAtlas>,
) {
    for (e, mut velocity, mut char_animation) in &mut query {
        if input.pressed(KeyCode::Space) {
            char_animation.animation_type = AnimationType::Attack;

            commands.entity(e).insert(SpriteSheetBundle {
                texture_atlas: spritesheets.player_atlas_2.clone(),
                ..default()
            });
        } else {
            // commands.entity(e).insert(SpriteSheetBundle {
            //     texture_atlas: spritesheets.player_atlas_1.clone(),
            //     sprite: TextureAtlasSprite::new(0),
            //     ..default()
            // });

            let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
            let left = if input.pressed(KeyCode::A) { 1. } else { 0. };
            let up = if input.pressed(KeyCode::W) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

            velocity.linvel.x = right - left;
            velocity.linvel.y = up - down;

            velocity.linvel = velocity.linvel.normalize_or_zero() * 100.;

            let linvel_norm = velocity.linvel.distance(Vec2::ZERO);
            if char_animation.animation_type != AnimationType::Attack {
                if linvel_norm == 0.0 {
                    char_animation.animation_type = AnimationType::Stand;
                } else {
                    char_animation.animation_type = AnimationType::Walk;
                }
            }

            if char_animation.animation_type == AnimationType::Walk {
                if velocity.linvel.x > 0. {
                    char_animation.direction = AnimationDirection::Right;
                } else if velocity.linvel.x < 0. {
                    char_animation.direction = AnimationDirection::Left;
                } else if velocity.linvel.y > 0. {
                    char_animation.direction = AnimationDirection::Up;
                } else if velocity.linvel.y < 0. {
                    char_animation.direction = AnimationDirection::Down;
                }
            }
        }
    }
}

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}
