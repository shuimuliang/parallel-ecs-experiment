use specs::System;
use specs::{Component, ReadStorage, VecStorage, WriteStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct StartPosition {
    x: i64,
    y: i64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct EndPosition {
    x: i64,
    y: i64,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: i64,
    y: i64,
}

struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, StartPosition>,
        ReadStorage<'a, EndPosition>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut start_positions, end_positions, v): Self::SystemData) {
        use specs::Join;

        for (start_pos, end_pos, v) in (&mut start_positions, &end_positions, &v).join() {
            let mut x_direction = 1_i64;
            let mut y_direction = 1_i64;
            let x_delta = end_pos.x - start_pos.x;
            let y_delta = end_pos.y - start_pos.y;
            if x_delta < 0 {
                x_direction = -1
            }
            if y_delta < 0 {
                y_direction = -1
            }

            if x_delta != 0 {
                start_pos.x += x_direction * v.x
            }
            if y_delta != 0 {
                start_pos.y += y_direction * v.y
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    #[test]
    fn test_movement() {
        let mut world = World::new();
        world.register::<StartPosition>();
        world.register::<EndPosition>();
        world.register::<Velocity>();

        let entity = world
            .create_entity()
            .with(StartPosition { x: 0, y: 0 })
            .with(EndPosition { x: 2, y: 4 })
            .with(Velocity { x: 1, y: 2 })
            .build();

        let mut movement_system = MovementSystem;
        movement_system.run_now(&world);

        let start_positions = world.write_storage::<StartPosition>();
        let start_pos = start_positions.get(entity).unwrap();
        assert_eq!(start_pos.x, 1);
        assert_eq!(start_pos.y, 2);

        let end_positions = world.read_storage::<EndPosition>();
        let end_pos = end_positions.get(entity).unwrap();
        assert_eq!(end_pos.x, 2);
        assert_eq!(end_pos.y, 4);

        let v = world.read_storage::<Velocity>();

        movement_system.run((start_positions, end_positions, v));

        let start_positions = world.write_storage::<StartPosition>();
        let start_pos = start_positions.get(entity).unwrap();
        assert_eq!(start_pos.x, 2);
        assert_eq!(start_pos.y, 4);
    }
}
