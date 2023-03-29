#[cfg(test)]
mod tests {

    use specs::WorldExt;

    #[derive(Default)]
    struct DeltaTime(f32);

    #[test]
    fn test_resource() {
        let mut world = specs::World::new();
        world.insert(DeltaTime(0.5));
        let delta_time = world.read_resource::<DeltaTime>();
        assert_eq!(delta_time.0, 0.5);
    }
}
