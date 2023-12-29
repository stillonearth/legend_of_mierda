use bevy::prelude::*;
use bevy_particle_systems::Particle;

pub fn fix_particle_transform_z(mut q: Query<(&mut Transform, &Particle)>) {
    for (mut transform, _) in q.iter_mut() {
        transform.translation.z = 100.0;
    }
}
