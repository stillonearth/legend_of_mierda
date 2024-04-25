//! Bevy Version: 0.10
//! Original: https://gist.github.com/GianpaoloBranca/17e5bd6ada9bdb04cca58182db8505d4
//! See also: https://github.com/bevyengine/bevy/issues/1515

use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;

pub struct CloneEntity {
    pub source: Entity,
    pub destination: Entity,
}

impl CloneEntity {
    // Copy all components from an entity to another.
    // Using an entity with no components as the destination creates a copy of the source entity.
    // Panics if:
    // - the components are not registered in the type registry,
    // - the world does not have a type registry
    // - the source or destination entity do not exist
    fn clone_entity(self, world: &mut World) {
        let components = {
            let registry = world.get_resource::<AppTypeRegistry>().unwrap().read();

            world
                .get_entity(self.source)
                .unwrap()
                .archetype()
                .components()
                .map(|component_id| {
                    world
                        .components()
                        .get_info(component_id)
                        .unwrap()
                        .type_id()
                        .unwrap()
                })
                .map(|type_id| {
                    let tp = registry.get(type_id);
                    tp?;
                    tp.unwrap().data::<ReflectComponent>()
                })
                .filter(|comp| comp.is_some())
                .map(|comp| comp.unwrap().clone())
                .collect::<Vec<_>>()
        };

        for component in components {
            let source = component
                .reflect(world.get_entity(self.source).unwrap())
                .unwrap()
                .clone_value();

            let mut destination = world.get_entity_mut(self.destination).unwrap();
            let mut type_registry = TypeRegistry::default();

            component.apply_or_insert(&mut destination, &*source, &type_registry);
        }
    }
}

// This allows the command to be used in systems
impl Command for CloneEntity {
    fn apply(self, world: &mut World) {
        self.clone_entity(world)
    }
}

// **********************************************************
// Adapt the code below to your needs
// **********************************************************

// Components must derive Reflect and use reflect(Component)
// If your component includes some other structs, these have to derive from Reflect and FromReflect
// #[derive(Component, Reflect, Default)]
// #[reflect(Component)]
// pub struct Foo {
//     pub bar: i32,
//     pub baz: bool,
// }

// // In your system, clone an entity like this:
// fn clone_entity(mut commands: Commands /* whatever you want that gets your entity */) {
//     // your code here to fetch the source entity
//     // ...
//     let copy = commands.spawn_empty().id();
//     commands.add(CloneEntity {
//         source: entity,
//         destination: copy,
//     });
// }

// // Also, remember to register your components in the App Type Registry!
// // Some predefined components are already registered (e.g. Name)
// // You can write your own plugin where you put all your registrations
// fn main() {
//     let mut app = App::new();

//     {
//         let registry = app.world.resource_mut::<AppTypeRegistry>();
//         let mut wr = registry.write();
//         wr.register::<Foo>();
//         // other structs...
//     }

//     app.run()
// }
