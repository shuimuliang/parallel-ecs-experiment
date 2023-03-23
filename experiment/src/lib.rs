use specs::{Component, Join, ReadStorage, System, VecStorage, WriteStorage};

#[derive(Component)]
#[storage(VecStorage)]
struct StartPosition {
    x: i64,
    y: i64,
}

#[derive(Component)]
#[storage(VecStorage)]
struct EndPosition {
    x: i64,
    y: i64,
}

struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, StartPosition>,
        WriteStorage<'a, EndPosition>,
    );

    fn run(&mut self, (start_positions, mut end_positions): Self::SystemData) {
        for (start_pos, end_pos) in (&start_positions, &mut end_positions).join() {
            // Move entity from start position to end position
            end_pos.x = start_pos.x + 10;
            end_pos.y = start_pos.y + 10;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{world::Builder, world::WorldExt, RunNow, World};

    #[test]
    fn test_movement() {
        let mut world = World::new();
        world.register::<StartPosition>();
        world.register::<EndPosition>();

        let entity = world
            .create_entity()
            .with(StartPosition { x: 0, y: 0 })
            .with(EndPosition { x: 0, y: 0 })
            .build();

        let mut movement_system = MovementSystem {};
        movement_system.run_now(&world);

        let end_positions = world.read_storage::<EndPosition>();
        let end_pos = end_positions.get(entity).unwrap();

        assert_eq!(end_pos.x, 10);
        assert_eq!(end_pos.y, 10);
    }
}
