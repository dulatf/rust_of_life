use std::collections::hash_map::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Loc {
    pub row: i64,
    pub col: i64,
}
impl Loc {
    pub fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
    pub fn neighbors(&self) -> [Loc; 8] {
        [
            Loc::new(self.row - 1, self.col - 1),
            Loc::new(self.row - 1, self.col),
            Loc::new(self.row - 1, self.col + 1),
            Loc::new(self.row, self.col - 1),
            Loc::new(self.row, self.col + 1),
            Loc::new(self.row + 1, self.col - 1),
            Loc::new(self.row + 1, self.col),
            Loc::new(self.row + 1, self.col + 1),
        ]
    }
}

type Buffer = HashMap<Loc, bool>;
fn is_alive(buffer: &Buffer, loc: &Loc) -> bool {
    *buffer.get(loc).unwrap_or(&false)
}

pub fn new_status(alive: bool, alive_neighbors: usize) -> bool {
    if alive && (alive_neighbors == 2 || alive_neighbors == 3) {
        true
    } else if !alive && alive_neighbors == 3 {
        true
    } else {
        false
    }
}

#[derive(Clone)]
pub struct World {
    buffer_1: Buffer,
    buffer_2: Buffer,
    using_buffer_1: bool,
}
impl Default for World {
    fn default() -> Self {
        Self {
            buffer_1: HashMap::new(),
            buffer_2: HashMap::new(),
            using_buffer_1: true,
        }
    }
}
impl World {
    pub fn from_configuration(
        data: &str,
        dead_char: char,
        alive_char: char,
    ) -> Result<Self, String> {
        let mut world = Self::default();
        let mut row = 0;
        let mut col = 0;
        let mut max_row = 0;
        let mut max_col = 0;
        for c in data.chars() {
            if c == dead_char || c == alive_char {
                max_col += 1;
            } else if c == '\n' {
                max_col = 0;
                max_row += 1;
            } else if c == '\r' {
            } else {
                return Err(format!("Invalid char {} at ({}, {})", c, row, col));
            }
        }

        for c in data.chars() {
            if c == dead_char {
                world.set(&Loc::new(row - max_row / 2, col - max_col / 2), false);
                col += 1;
            } else if c == alive_char {
                world.set(&Loc::new(row - max_row / 2, col - max_col / 2), true);
                col += 1;
            } else if c == '\n' {
                row += 1;
                col = 0;
            } else if c == '\r' {
            } else {
                return Err(format!("Invalid char {} at ({}, {})", c, row, col));
            }
        }
        Ok(world)
    }
    pub fn current_buffer(&self) -> &Buffer {
        if self.using_buffer_1 {
            &self.buffer_1
        } else {
            &self.buffer_2
        }
    }
    pub fn next_buffer(&mut self) -> &mut Buffer {
        if self.using_buffer_1 {
            &mut self.buffer_2
        } else {
            &mut self.buffer_1
        }
    }
    pub fn get(&self, loc: &Loc) -> bool {
        is_alive(self.current_buffer(), loc)
    }
    pub fn set(&mut self, loc: &Loc, alive: bool) {
        let buf = self.next_buffer();
        match buf.get_mut(loc) {
            Some(val) => *val = alive,
            None => {
                buf.insert(*loc, alive);
            }
        }
        if alive {
            for nb in loc.neighbors().iter() {
                if buf.get(nb).is_none() {
                    buf.insert(*nb, false);
                }
            }
        }
    }
    pub fn step(&mut self) {
        let locs: Vec<Loc> = self.current_buffer().keys().map(|&loc| loc).collect();
        for loc in locs.iter() {
            let alive = self.get(&loc);
            let neighbors = loc.neighbors();
            let alive_neighbors = neighbors
                .iter()
                .map(|neighbor| is_alive(self.current_buffer(), neighbor))
                .filter(|&alive| alive)
                .count();
            if alive_neighbors > 0 {
                self.set(&loc, new_status(alive, alive_neighbors));
            }
        }
        self.using_buffer_1 = !self.using_buffer_1;
        self.next_buffer().clear();
    }
}
