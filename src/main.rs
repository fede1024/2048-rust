extern crate time;

mod world;

use std::cmp;
use time::PreciseTime;

use world::{Dir, LineWorld16, Tile, World};

fn best_tile<W>(world: &W) -> i32
    where W: for <'a> World<'a>
{
    world.iterate().fold(0i32, |best, (_, v)| cmp::max(v.to_i32(), best))
}

fn total<W>(world: &W) -> i32
    where W: for <'a> World<'a>
{
    world.iterate().fold(0i32, |sum, (_, v)| sum + v.to_i32())
}

trait Heuristic {
    fn call<W: for<'a> World<'a>>(&self, &W) -> i32;
    fn description(&self) -> &'static str;
}

struct HEmpty;
impl Heuristic for HEmpty {
    fn call<W: for<'a> World<'a>>(&self, world: &W) -> i32 {
        world.iterate().filter(|&(_, t)| t.empty()).count() as i32
    }
    fn description(&self) -> &'static str {
        "Empty count"
    }
}

struct HSum;
impl Heuristic for HSum {
    fn description(&self) -> &'static str {
        "Total sum"
    }
    fn call<W: for<'a> World<'a>>(&self, world: &W) -> i32 {
        world.iterate().fold(0i32, |sum, (_, v)| {
            let n = v.to_i32();
            if n != 0 { sum + n } else { sum + 256 }
        })
    }
}

struct HSquared;
impl Heuristic for HSquared {
    fn description(&self) -> &'static str {
        "Squared"
    }
    fn call<W: for<'a> World<'a>>(&self, world: &W) -> i32 {
        world.iterate().fold(0i32, |sum, (_, v)| sum + v.to_i32().pow(2))
    }
}

fn alphabeta<W, F>(w: &W, depth: i32, mut alpha: i32, mut beta: i32, max_p: bool, moved: bool, h: &F) -> (Dir, i32)
    where W: for<'a> World<'a>,
          F: Heuristic,
{
    if depth <= 0 || !moved {
        return (Dir::Up, h.call(w));
    }

    let empty_tiles = w.iterate().filter(|&(_, t)| t.empty()).count();
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
            if !v.empty() {
                continue;
            }
            for &x in [2, 4].iter() {
                let ref mut w1 = w.clone();
                w1.set(c, W::Tile::from_i32(x));
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
    let start_time = PreciseTime::now();
    let mut count = 0;
    let mut last_print = -1;

    let h = HSquared;
    let depth = 9;

    while world::add_rand_tile::<LineWorld16, usize>(&mut world) {
        let (d, v) = alphabeta(&world, depth, std::i32::MIN, std::i32::MAX, true, true, &h);
        world.do_move(d);
        count += 1;
        let duration_s = start_time.to(PreciseTime::now()).num_seconds();
        if duration_s != last_print {
            last_print = duration_s;
            println!("> {:?} {}", d, v);
            world.print();
        }
    }

    println!("Three depth: {}", depth);
    println!("Using heuristic: {}\n", h.description().to_string());
    world.print();

    let duration = start_time.to(PreciseTime::now());
    println!("{} moves in {}", count, duration);
    println!("{} moves per second", count / cmp::max(1, duration.num_seconds()));

    println!("{} total, {} best", total(&world), best_tile(&world));
}
