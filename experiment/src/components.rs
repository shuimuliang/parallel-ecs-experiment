use {
    specs::{Component, VecStorage},
    std::{
        collections::HashMap,
        hash::Hash,
        ops::{Add, Sub},
    },
};

#[derive(Component, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[storage(VecStorage)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }

    pub fn append_x(&mut self, x: i64) {
        self.x += x
    }

    pub fn append_y(&mut self, y: i64) {
        self.y += y
    }

    pub fn append_xy(&mut self, x: i64, y: i64) {
        self.x += x;
        self.y += y;
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y)
    }
}

// It is very important that you don't request both a ReadStorage and a WriteStorage for the same component
// or a Read and a Write for the same resource.
// This is just like the borrowing rules of Rust, where you can't borrow something mutably and immutably at the same time.
// In Specs, we have to check this at runtime, thus you'll get a panic if you don't follow this rule.

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct StartPosition {
    pub point: Pos,
}

impl StartPosition {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            point: Pos::new(x, y),
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct EndPosition {
    pub point: Pos,
}

impl EndPosition {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            point: Pos::new(x, y),
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: u64,
    pub y: u64,
}

impl Velocity {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AssetKind {
    Apple = 0,
    Banana = 1,
    Orange = 2,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Asset {
    kind: AssetKind,
    amount: u64,
}

impl Asset {
    #[allow(dead_code)]
    fn new(kind: AssetKind, amount: u64) -> Self {
        Asset { kind, amount }
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Inventory {
    assets: HashMap<AssetKind, u64>,
}

impl Inventory {
    pub fn add_asset(&mut self, asset: Asset) {
        *self.assets.entry(asset.kind).or_insert(asset.amount) += asset.amount;
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Map {
    assets: HashMap<Pos, Asset>,
}

impl Map {
    #[allow(dead_code)]
    fn remove_asset(&mut self, pos: Pos) -> Option<Asset> {
        if let Some(asset) = self.assets.remove(&pos) {
            return Some(asset);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::components::{Asset, AssetKind, Map, Pos};

    #[test]
    fn test_map() {
        let asset_apple = Asset::new(AssetKind::Apple, 5);
        let mut map = Map::default();
        let pos = Pos::new(2, 4);
        map.assets.insert(pos, asset_apple);

        let v = map.remove_asset(pos);
        assert!(v.is_some());

        {
            let v = v.unwrap();
            matches!(v.kind, AssetKind::Apple);
            assert_eq!(v.amount, 5);
        }
    }
}
