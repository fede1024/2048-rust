extern crate rand;

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn generate_new_tile_value() -> i32 {
    match rand::random::<i32>() % 10 {
        9 => 4,
        _ => 2,
    }
}

pub trait Tile: Copy + Eq + fmt::Display {
    fn to_i32(&self) -> i32;
    fn from_i32(i32) -> Self;

    fn empty(&self) -> bool {
        self.to_i32() == 0
    }
}

pub trait Coord: Copy {
    fn to_xy(&self) -> (usize, usize);
    fn from_xy(usize, usize) -> Self;
}

// TODO: add get size
pub trait World<'a>: Clone {
    type Tile: Tile;
    type Coord: Coord;
    type Iter: Iterator<Item=(Self::Coord, Self::Tile)>;

    fn new() -> Self;
    fn get(&self, usize, usize) -> Self::Tile;
    fn set(&mut self, Self::Coord, Self::Tile);
    fn do_move(&mut self, Dir) -> bool;
    fn iterate(&'a self) -> Self::Iter;

    fn print(&self) {
        println!("┌────┬────┬────┬────┐");
        for y in 0..4 {
            for x in 0..4 {
                let c = self.get(x, y);
                if c.empty() {
                    print!("│    ");
                } else {
                    print!("│{:4}", c);
                }
            }
            if y < 3 {
                println!("│\n├────┼────┼────┼────┤");
            } else {
                println!("│\n└────┴────┴────┴────┘");
            }
        }
    }
}

pub fn add_rand_tile<W, T>(world: &mut W) -> bool
    where W: for<'a> World<'a, Coord=T>,
          T: Coord,
{
    let empty_tiles = world.iterate().filter(|&(_, t)| t.empty()).count();
    if empty_tiles == 0 {
        return false;
    }

    let mut p = rand::random::<usize>() % empty_tiles;
    let (coord, _) = world.iterate().filter(|&(_, t)| t.empty()).nth(p).unwrap();
    world.set(coord, Tile::from_i32(generate_new_cell_value()));

    return true;
}
