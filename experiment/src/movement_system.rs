use {
    crate::components::{EndPosition, StartPosition, Velocity},
    specs::{Entities, Join, ReadStorage, System, WriteStorage},
};

// a entity has a start position and end position, and velocity
// create a movement system, let entity move from start position to end position

// TODO: MovementSystem for Multi-entity collisions
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, StartPosition>,
        ReadStorage<'a, EndPosition>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, mut start_pos, end_pos, v): Self::SystemData) {
        for (entity, start_pos, end_pos, v) in (&entities, &mut start_pos, &end_pos, &v).join() {
            let mut x_direction = 1_i64;
            let mut y_direction = 1_i64;
            let x_delta = end_pos.point.x - start_pos.point.x;
            let y_delta = end_pos.point.y - start_pos.point.y;
            if x_delta < 0 {
                x_direction = -1
            }
            if y_delta < 0 {
                y_direction = -1
            }

            if x_delta != 0 {
                start_pos.point.append_x(x_direction * (v.x as i64));
            }
            if y_delta != 0 {
                start_pos.point.append_y(y_direction * (v.y as i64));
            }
            dbg!(entity.id());
        }
        dbg!("movement once");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, DispatcherBuilder, RunNow, World, WorldExt};

    #[test]
    fn test_movement() {
        let mut world = World::new();
        world.register::<StartPosition>();
        world.register::<EndPosition>();
        world.register::<Velocity>();
        //
        let entity = world
            .create_entity()
            .with(StartPosition::new(0, 0))
            .with(EndPosition::new(2, 4))
            .with(Velocity::new(1, 2))
            .build();

        let mut movement_system = MovementSystem;
        movement_system.run_now(&world);

        let start_pos_comp = world.write_storage::<StartPosition>();
        let start_pos = start_pos_comp.get(entity).unwrap();
        assert_eq!(start_pos.point.x, 1);
        assert_eq!(start_pos.point.y, 2);

        let end_pos_comp = world.read_storage::<EndPosition>();
        let end_pos = end_pos_comp.get(entity).unwrap();
        assert_eq!(end_pos.point.x, 2);
        assert_eq!(end_pos.point.y, 4);

        let v = world.read_storage::<Velocity>();
        let entities = world.entities();
        movement_system.run((entities, start_pos_comp, end_pos_comp, v));

        let start_pos_comp = world.write_storage::<StartPosition>();
        let start_pos = start_pos_comp.get(entity).unwrap();
        assert_eq!(start_pos.point.x, 2);
        assert_eq!(start_pos.point.y, 4);
    }

    #[test]
    fn test_dispatcher_no_dependency() {
        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem, "movement", &[])
            .build();

        let mut world = World::new();
        world.register::<StartPosition>();
        world.register::<EndPosition>();
        world.register::<Velocity>();

        let entity = world
            .create_entity()
            .with(StartPosition::new(0, 0))
            .with(EndPosition::new(2, 4))
            .with(Velocity::new(1, 2))
            .build();

        dispatcher.dispatch(&mut world);
        {
            let start_pos_comp = world.write_storage::<StartPosition>();
            let start_pos = start_pos_comp.get(entity).unwrap();
            assert_eq!(start_pos.point.x, 1);
            assert_eq!(start_pos.point.y, 2);
        }
        dispatcher.dispatch(&mut world);
        {
            let start_pos_comp = world.write_storage::<StartPosition>();
            let start_pos = start_pos_comp.get(entity).unwrap();
            assert_eq!(start_pos.point.x, 2);
            assert_eq!(start_pos.point.y, 4);
        }
    }

    struct FinalReportSystem;

    impl<'a> System<'a> for FinalReportSystem {
        type SystemData = (Entities<'a>, ReadStorage<'a, EndPosition>);

        fn run(&mut self, (entities, end_pos): Self::SystemData) {
            for (entity, e) in (&entities, &end_pos).join() {
                dbg!(entity);
                dbg!(&e);
            }
        }
    }

    #[test]
    fn test_dispatcher_say_goodbye() {
        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem, "movement", &[])
            .with(FinalReportSystem, "final_report", &["movement"])
            .build();

        let mut world = World::new();
        world.register::<StartPosition>();
        world.register::<EndPosition>();
        world.register::<Velocity>();
        //
        let _entity = world
            .create_entity()
            .with(StartPosition::new(0, 0))
            .with(EndPosition::new(2, 4))
            .with(Velocity::new(1, 2))
            .build();

        dispatcher.dispatch(&mut world);
    }

    #[test]
    fn test_dispatcher_say_goodbye_to_entities() {}

    #[test]
    fn query_entity_contain_position_component() {
        struct QuerySystem;

        impl<'a> System<'a> for QuerySystem {
            type SystemData = (Entities<'a>, ReadStorage<'a, StartPosition>);

            fn run(&mut self, (entities, _start_pos): Self::SystemData) {
                let mut v = Vec::new();
                for (entity, _) in (&entities, &_start_pos).join() {
                    v.push(entity.id());
                }
                dbg!(&v);
            }
        }

        let mut world = World::new();
        world.register::<StartPosition>();
        world.register::<EndPosition>();
        world.register::<Velocity>();

        let _entity1 = world
            .create_entity()
            .with(StartPosition::new(0, 0))
            .with(EndPosition::new(2, 4))
            .with(Velocity::new(1, 2))
            .build();

        let _entity2 = world.create_entity().with(EndPosition::new(2, 4)).build();

        let mut query_system = QuerySystem;
        query_system.run_now(&world);
    }
}
