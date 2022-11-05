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

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>, 
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (dt, vel, mut pos): Self::SystemData) {

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * dt.0;
            pos.y += vel.y * dt.0;
        }
    }
}

#[derive(Default)]
struct DeltaTime(f32);

fn main() {
    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();

    world.insert(DeltaTime(0.0));

    let _ = world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Velocity { x: 0.5, y: 0.0 })
        .build();
    let _ = world
        .create_entity()
        .with(Position { x: 10.0, y: 11.0 })
        .with(Velocity { x: 0.5, y: 0.0 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();

    let mut dt = world.write_resource::<DeltaTime>();
    *dt = DeltaTime(0.05);
    // drop(dt);

    dispatcher.dispatch(&mut world);
    world.maintain();
}
