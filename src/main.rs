extern crate time;

mod world;

use std::cmp;
use time::PreciseTime;

use world::{Dir, LineWorld16, Coord, Tile, World};

fn best_tile<W>(world: &W) -> i32
    where W: for <'a> World<'a>
{
    let mut best = 0;
    for (_, v) in world.iterate() {
        if !v.empty() {
            best = cmp::max(v.to_i32(), best);
        }
    }
    best
}

fn total<W>(world: &W) -> i32
    where W: for <'a> World<'a>
{
    let mut total = 0;
    for (_, v) in world.iterate() {
        total += v.to_i32();
    }
    total
}

fn h1<W>(world: &W) -> i32
    where W: for<'a> World<'a>
{
    let mut empty = 0;
    for (_, v) in world.iterate() {
        if v.empty() {
            empty += 1;
        }
    }
    empty
}

fn h2<W>(world: &W) -> i32
    where W: for<'a> World<'a, Tile = i32>,
{
    let mut sum = 0;
    for (_, v) in world.iterate() {
        if v != 0 {
            sum += v;
        } else {
            sum += 256;
        }
    }
    sum
}

fn h3<W>(world: &W) -> i32
    where W: for<'a> World<'a, Tile = i32>
{
    let mut sum = 0;
    for (_, v) in world.iterate() {
        if v != 0 {
            sum += v * v;
        }
    }
    sum
}

fn alphabeta<W, F>(w: &W, depth: i32, mut alpha: i32, mut beta: i32, max_p: bool, moved: bool, h: &F) -> (Dir, i32)
    where W: for<'a> World<'a, Tile = i32>,
          F: Fn(&W) -> i32,
{
    if depth <= 0 || !moved {
        return (Dir::Up, h(w));
    }

    let mut empty_tiles = 0;
    for (_, v) in w.iterate() {
        if v == 0 {
            empty_tiles += 1;
        }
    }
    let new_depth = if empty_tiles > 8 { depth / 2 } else { depth };

    if max_p {
        let mut best: (Dir, i32) = (Dir::Up, std::i32::MIN);
        for &d in [Dir::Up, Dir::Down, Dir::Left, Dir::Right].iter() {
            let ref mut w1 = w.clone();
            let moved = w1.do_move(d);
            let (_, val) = alphabeta(w1, new_depth - 1, alpha, beta, false, moved, h);
            if best.1 < val {
                best = (d, val);
            }
            alpha = cmp::max(alpha, best.1);
            if beta <= alpha {
                break;
            }
        }
        return best;
    } else {
        let mut best: (Dir, i32) = (Dir::Up, std::i32::MAX);
        for (c, v) in w.iterate() {
            if v != 0 {
                continue;
            }
            for &x in [2, 4].iter() {
                let ref mut w1 = w.clone();
                w1.set(c, x);
                let (_, val) = alphabeta(w1, new_depth - 1, alpha, beta, true, true, h);
                if best.1 > val {
                    best = (Dir::Up, val);
                }
                beta = cmp::min(beta, best.1);
                if beta <= alpha {
                    break;
                }
            }
        }
        return best;
    }
}

fn main() {
    let mut world = LineWorld16::new();
    let mut count = 0;
    let start_time = PreciseTime::now();
    let mut last_print = 0;

    // TODO: is there a better way to do this?
    while world::add_rand_tile::<LineWorld16, usize>(&mut world) {
        let (d, v) = alphabeta(&world, 9, std::i32::MIN, std::i32::MAX, true, true, &h3);
        world.do_move(d);
        count += 1;
        let duration_s = start_time.to(PreciseTime::now()).num_seconds();
        if duration_s != last_print {
            last_print = duration_s;
            println!("> {:?} {}", d, v);
            world.print();
        }
    }
    world.print();

    let duration = start_time.to(PreciseTime::now());
    println!("{} moves in {}", count, duration);
    println!("{} moves per second", count / cmp::max(1, duration.num_seconds()));

    println!("{} total, {} best", total(&world), best_tile(&world));
}
