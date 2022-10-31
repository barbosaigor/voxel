use specs::{prelude::*, Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}


fn main() {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();

    let _ = world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    let _ = world.create_entity().with(Position { x: 10.0, y: 11.0 }).build();

    let mut hello_world = HelloWorld;
    hello_world.run_now(&world);
    let _ = world.create_entity().with(Position { x: 10.0, y: 12.0 }).build();
    world.maintain();
    hello_world.run_now(&world);
}
