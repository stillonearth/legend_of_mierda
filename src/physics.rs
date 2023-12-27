use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::*, events::*};

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

pub(crate) fn handle_player_pizza_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut q_pizzas: Query<(Entity, &Pizza)>,
    q_player: Query<(Entity, &mut Player)>,
    mut ev_pizza_step_over: EventWriter<PizzaStepOverEvent>,
) {
    for (player_entity, _) in q_player.iter() {
        for event in collision_events.iter() {
            for (e_pizza, _) in q_pizzas.iter_mut() {
                if let CollisionEvent::Started(e1, e2, _) = event {
                    if e1.index() == e_pizza.index() && e2.index() == player_entity.index() {
                        ev_pizza_step_over.send(PizzaStepOverEvent(e_pizza));
                        return;
                    }

                    if e2.index() == e_pizza.index() && e1.index() == player_entity.index() {
                        ev_pizza_step_over.send(PizzaStepOverEvent(e_pizza));
                        return;
                    }
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
                if q_los_mierdas.get(other_entity).is_err() {
                    continue;
                }

                ev_player_hit.send(PlayerHitEvent { entity: e });
            }
        }
    }
}
