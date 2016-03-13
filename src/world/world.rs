extern crate rand;

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn generate_new_cell_value() -> i32 {
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

// TODO: add get size
pub trait World<'a>: Clone {
    type Cell: Tile;
    type Coord;
    type Iter: Iterator<Item=(Self::Coord, Self::Cell)>;

    fn new() -> Self;
    fn get(&self, usize, usize) -> Self::Cell;
    fn set(&mut self, Self::Coord, Self::Cell);
    fn do_move(&mut self, Dir) -> bool;
    fn iterate(&'a self) -> Self::Iter;

    fn to_coord(usize, usize) -> Self::Coord;
    fn from_coord(Self::Coord) -> (usize, usize);

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

pub fn add_rand_cell<W, T>(world: &mut W) -> bool
    where W: for<'a> World<'a, Coord = T>,
          T: Copy,
{
    let empty_cells = world.iterate().filter(|&(_, tile)| tile.empty()).count();
    if empty_cells == 0 {
        return false;
    }

    let mut p = rand::random::<usize>() % empty_cells;
    let (coord, _) = world.iterate().filter(|&(_, tile)| tile.empty()).nth(p).unwrap();
    world.set(coord, Tile::from_i32(generate_new_cell_value()));

    return true;
}
