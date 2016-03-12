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

// TODO: add get size
pub trait World<'a>: Clone {
    type Cell: Eq + fmt::Display + Copy;
    type Coord;
    type Iter: Iterator<Item=(Self::Coord, Self::Cell)>;

    fn new() -> Self;
    fn get(&self, usize, usize) -> Self::Cell;
    fn set(&mut self, Self::Coord, Self::Cell);
    fn do_move(&mut self, Dir) -> bool;
    fn iterate(&'a self) -> Self::Iter;

    fn to_cell(v: i32) -> Self::Cell;
    fn from_cell(Self::Cell) -> i32;
    fn to_coord(usize, usize) -> Self::Coord;
    fn from_coord(Self::Coord) -> (usize, usize);

    fn empty_cell(c: Self::Cell) -> bool {
        Self::from_cell(c) == 0
    }

    fn print(&self) {
        println!("+----+----+----+----+");
        for y in 0..4 {
            for x in 0..4 {
                let c = self.get(x, y);
                if Self::empty_cell(c) {
                    print!("|    ");
                } else {
                    print!("|{:4}", c);
                }
            }
            println!("|\n+----+----+----+----+");
        }
    }
}

pub fn add_rand_cell<W>(world: &mut W) -> bool
    where W: for<'a> World<'a>
{
    let cell = W::to_cell(generate_new_cell_value());

    let mut empty_cells = 0;
    for (_, v) in world.iterate() {
        if W::empty_cell(v) {
            empty_cells += 1;
        }
    }

    if empty_cells == 0 {
        return false;
    }

    let mut p = rand::random::<usize>() % empty_cells + 1;
    let mut final_c = W::to_coord(0, 0);
    for (c, v) in world.iterate() {
        if W::empty_cell(v) {
            p -= 1;
        }
        if p == 0 {
            final_c = c;
            break;
        }
    }
    world.set(final_c, cell);

    return true;
}
