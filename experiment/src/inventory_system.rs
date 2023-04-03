// Inventory System
// Entity: cow
// Components: entity_id, EndPosition{point}, Inventory HashMap<AssetKind, u64>, Global AssetMap
// Description:
// a cow entity finally moved to EndPosition in MoveSystem
// if it is on an apple Asset, take the apple into the inventory component
// movement_system 1st, inventory_system 2nd

use {
    crate::components::{EndPosition, Inventory, Map},
    specs::{ReadStorage, System, WriteStorage},
};

pub struct InventorySystem;

impl<'a> System<'a> for InventorySystem {
    type SystemData = (
        ReadStorage<'a, EndPosition>,
        WriteStorage<'a, Inventory>,
        ReadStorage<'a, Map>,
    );

    fn run(&mut self, _: Self::SystemData) {}
    // fn run(&mut self, (end_pos, mut assets, mut map): Self::SystemData) {
    //     for (end_pos, asset, map) in (&end_pos, &assets, &map).join() {
    //         // Check if entity is on an asset
    //         // move asset to inventory
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::components::{EndPosition, Inventory, Map, StartPosition, Velocity};
    use crate::inventory_system::InventorySystem;
    use crate::movement_system::MovementSystem;
    use specs::{Builder, DispatcherBuilder, World, WorldExt};

    #[test]
    fn test_inventory_system() {
        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSystem, "movement", &[])
            .with(InventorySystem, "inventory", &["movement"])
            .build();

        let mut world = World::new();
        world.register::<StartPosition>();
        world.register::<EndPosition>();
        world.register::<Velocity>();
        world.register::<Map>();
        world.register::<Inventory>();

        let _entity = world
            .create_entity()
            .with(StartPosition::new(0, 0))
            .with(EndPosition::new(2, 4))
            .with(Velocity::new(1, 2))
            .build();

        dispatcher.dispatch(&mut world);
    }
}
