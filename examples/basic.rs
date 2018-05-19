extern crate specs;
extern crate specs_static;

use specs::{
    Component, DispatcherBuilder, ReadExpect, Read, Write, Join, System, SystemData, VecStorage, World,
};
use specs_static::{Id, Storage, WorldExt};

pub struct Tiles {
    width: usize,
    height: usize,
}

impl Tiles {
    pub fn new(width: usize, height: usize) -> Self {
        Tiles { width, height }
    }

    pub fn id(&self, x: usize, y: usize) -> TileId {
        TileId(y as u32 * self.width as u32 + x as u32)
    }

    pub fn iter_all(&self) -> Box<Iterator<Item = TileId>> {
        Box::new((0..self.width as u32 * self.height as u32 - 1).map(TileId))
    }
}

pub type TileComps<'a, C> = Read<'a, Storage<C, <C as Component>::Storage, TileId>>;
pub type TileCompsMut<'a, C> = Write<'a, Storage<C, <C as Component>::Storage, TileId>>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TileId(u32);

impl Id for TileId {
    fn from_u32(value: u32) -> Self {
        TileId(value)
    }

    fn id(&self) -> u32 {
        self.0
    }
}

// ------

#[derive(Debug)]
enum Material {
    Dirt,
    Grass,
    Water,
}

impl Component for Material {
    type Storage = VecStorage<Self>;
}

struct Sys;

impl<'a> System<'a> for Sys {
    type SystemData = (ReadExpect<'a, Tiles>, TileComps<'a, Material>);

    fn run(&mut self, (tiles, materials): Self::SystemData) {
        if let Some(mat) = materials.get(tiles.id(3, 4)) {
            println!("The material at (3, 4) is {:?}.", mat);
        }

        let num_water: usize = (&*materials)
            .join()
            .map(|mat| match *mat {
                Material::Water => 1,
                _ => 0,
            })
            .sum();

        println!("There are {} tiles with water.", num_water);
    }
}

fn main() {
    let mut d = DispatcherBuilder::new().with(Sys, "sys", &[]).build();
    let mut w = World::new();

    // Use method provided by `WorldExt`.
    w.add_resource(Tiles::new(8, 8));
    w.register_tile_comp::<Material, TileId>();

    // Initialize

    {
        let tiles = w.read_resource::<Tiles>();
        let mut materials: TileCompsMut<Material> = SystemData::fetch(&w.res);

        for tile_id in tiles.iter_all() {
            materials.insert(tile_id, Material::Dirt);
        }

        materials.insert(tiles.id(1, 5), Material::Grass);
        materials.insert(tiles.id(2, 5), Material::Grass);
        materials.insert(tiles.id(3, 4), Material::Water);
        materials.insert(tiles.id(3, 7), Material::Water);
    }

    // ---

    d.dispatch(&mut w.res);
}
