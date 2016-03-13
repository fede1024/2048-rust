use world::{Dir, World};

#[derive(Copy, Clone, Debug)]
pub struct LineWorld16 {
    pub data: [i32; 16],
}

struct Iter16 {
    start: i32,
    step: i32,
    off: i32,
    count: i32,
}

impl Iter16 {
    fn new(dir: Dir, off: i32, skip: i32) -> Iter16 {
        match dir {
            Dir::Up    => Iter16 { start: 0, step: 4, off: off, count: skip },
            Dir::Down  => Iter16 { start: 12, step: -4, off: off, count: skip },
            Dir::Left  => Iter16 { start: 0, step: 1, off: off * 4, count: skip },
            Dir::Right => Iter16 { start: 3, step: -1, off: off * 4, count: skip },
        }
    }
}

impl Iterator for Iter16 {
    type Item = (i32, usize);

    fn next(&mut self) -> Option<(i32, usize)> {
        if self.count < 4 {
            let pair = (self.count, (self.start + self.count * self.step + self.off) as usize);
            self.count += 1;
            Some(pair)
        } else {
            None
        }
    }
}

impl LineWorld16 {
    fn squash_line(&mut self, dir: Dir, line: i32) {
        for (n, i) in Iter16::new(dir, line, 0) {
            if self.data[i] != 0 {
                continue;
            }
            for (_, j) in Iter16::new(dir, line, n) {
                if self.data[j] != 0 {
                    self.data[i] = self.data[j];
                    self.data[j] = 0;
                    break;
                }
            }
        }
    }

    fn merge_line(&mut self, dir: Dir, line: i32) {
        let mut prev: Option<usize> = None;
        for (_, i) in Iter16::new(dir, line, 0) {
            match prev {
                Some(p) => {
                    if self.data[p] == self.data[i] {
                        self.data[p] = self.data[p] * 2;
                        self.data[i] = 0;
                    }
                }
                None => (),
            };
            prev = Some(i);
        }
    }
}

struct LineWorld16Iter<'a> {
    world: &'a LineWorld16,
    count: usize,
}

impl<'a> Iterator for LineWorld16Iter<'a> {
    type Item = (usize, i32);
    fn next(&mut self) -> Option<(usize, i32)> {
        while self.count < 16 {
            let c = self.count;
            self.count += 1;
            return Some((c, self.world.data[c]));
        }
        None
    }
}

impl<'a> World<'a> for LineWorld16 {
    type Cell = i32;
    type Coord = usize;
    type Iter = LineWorld16Iter<'a>;

    fn new() -> LineWorld16 {
        LineWorld16 { data: [0; 16] }
    }

    fn get(&self, x: usize, y: usize) -> i32 {
        self.data[Self::to_coord(x, y)]
    }

    fn to_cell(v: i32) -> i32 { v }

    fn from_cell(c: i32) -> i32 { c }

    fn to_coord(x: usize, y: usize) -> usize {
        y * 4 + x
    }

    fn from_coord(c: usize) ->  (usize, usize) {
        (c % 4, c / 4)
    }

    fn set(&mut self, n: usize, val: i32) {
        self.data[n] = val;
    }

    fn do_move(&mut self, dir: Dir) -> bool {
        let old_data = self.data.clone();
        for line in 0..4 {
            self.squash_line(dir, line);
            self.merge_line(dir, line);
            self.squash_line(dir, line);
        }
        return old_data != self.data;
    }

    fn iterate(&'a self) -> LineWorld16Iter<'a> {
        LineWorld16Iter {
            world: self,
            count: 0,
        }
    }
}
