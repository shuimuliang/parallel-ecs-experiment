
# parallel design in specs

* The `Storage` component is pre-defined as readable or writable.

```rust
type SystemData = (
        Entities<'a>,
        WriteStorage<'a, StartPosition>,
        ReadStorage<'a, EndPosition>,
        ReadStorage<'a, Velocity>,
    );

```

This means that there can be **n** readable components and 1 writable component.

* **Parallel Join**

for readable component

use rayon::prelude::*;

```rust
fn run(&mut self, (vel, mut pos): Self::SystemData) {
    use rayon::prelude::*;
    use specs::ParJoin;

    // Parallel joining behaves similarly to normal joining
    // with the difference that iteration can potentially be
    // executed in parallel by a thread pool.
    (&vel, &mut pos)
        .par_join()
        .for_each(|(vel, pos)| {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
        });
}
```

* fork-join model

```rust
let mut dispatcher = DispatcherBuilder::new()
        .with(UpdatePos, "update_pos", &[])
        .with(CollectInventory, "collect_inventory", &["update_pos"])
        .build();
```

After reaching the final point, collect the inventory located there.
