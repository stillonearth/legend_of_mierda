use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::*, events::PlayerHitEvent};

pub(crate) fn handle_mierda_wall_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_los_mierdas: Query<(Entity, &mut Velocity, &Mierda)>,
) {
    for event in collision_events.iter() {
        for (e, mut v, _) in q_los_mierdas.iter_mut() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if e1.index() == e.index() || e2.index() == e.index() {
                    v.linvel *= -1.;
                }
            }
        }
    }
}

pub(crate) fn handle_player_mierda_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_player: Query<(Entity, &mut Player)>,
    q_los_mierdas: Query<(Entity, &mut Velocity, &Mierda)>,
    mut ev_player_hit: EventWriter<PlayerHitEvent>,
) {
    for event in collision_events.iter() {
        for (e, _) in q_player.iter_mut() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if !(e1.index() == e.index() || e2.index() == e.index()) {
                    continue;
                }

                let other_entity = if e1.index() == e.index() { *e2 } else { *e1 };
                if !q_los_mierdas.get(other_entity).is_ok() {
                    continue;
                }

                ev_player_hit.send(PlayerHitEvent { entity: e });
            }
        }
    }
}
