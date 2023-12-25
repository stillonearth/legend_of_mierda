use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

use crate::components::*;

pub(crate) fn mierda_activity(
    time: Res<Time>,
    mut los_mierdas: Query<(&mut Velocity, &mut Mierda)>,
) {
    for (mut v, mut mierda) in los_mierdas.iter_mut().filter(|(_, m)| !m.is_dummy) {
        let rotation_angle = time.elapsed_seconds().cos() * std::f32::consts::FRAC_PI_4;

        if mierda.hit_at.is_some() {
            let timer = mierda.hit_at.as_mut().unwrap();
            timer.tick(time.delta());
            if !timer.finished() {
                continue;
            } else {
                mierda.hit_at = None;
            }
        }
        v.linvel = Vec2::new(
            mierda.move_direction.x * rotation_angle.cos()
                - mierda.move_direction.y * rotation_angle.sin(),
            mierda.move_direction.x * rotation_angle.sin()
                + mierda.move_direction.y * rotation_angle.cos(),
        ) * 30.0;
    }
}

pub(crate) fn update_mierdas_move_direction(
    time: Res<Time>,
    player: Query<(&Transform, &Player)>,
    mut los_mierdas: Query<(&Transform, &mut DirectionUpdateTime, &mut Mierda)>,
) {
    if player.iter().count() == 0 {
        return;
    }

    let player_position = player.single().0.translation;

    for (mierda_position, mut direction_update_timer, mut mierda) in
        los_mierdas.iter_mut().filter(|(_, _, m)| !m.is_dummy)
    {
        direction_update_timer.timer.tick(time.delta());

        if direction_update_timer.timer.finished() || mierda.move_direction == Vec2::ZERO {
            let mierda_position = mierda_position.translation;
            mierda.move_direction = Vec2::new(
                player_position.x - mierda_position.x,
                player_position.y - mierda_position.y,
            )
            .normalize_or_zero();
        }
    }
}
