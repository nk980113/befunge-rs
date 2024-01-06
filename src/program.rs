pub struct Program {
    field: [[u8; 80]; 25],
    pc: (i8, i8),
    dir: Direction,
    stack: Vec<i64>,
}

impl Program {
    pub fn from_field(field: [[u8; 80]; 25]) -> Self {
        Self {
            field,
            pc: (0, 0),
            dir: Direction::Right,
            stack: Vec::new(),
        }
    }

    pub fn current_point(&self) -> u8 {
        self.field[self.pc.1 as usize][self.pc.0 as usize]
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }

    pub fn step(&mut self) {
        match self.dir {
            Direction::Up => self.pc.1 = (self.pc.1 - 1).rem_euclid(25),
            Direction::Down => self.pc.1 = (self.pc.1 + 1).rem_euclid(25),
            Direction::Left => self.pc.0 = (self.pc.0 - 1).rem_euclid(80),
            Direction::Right => self.pc.0 = (self.pc.0 + 1).rem_euclid(80),
        }
    }

    pub fn point(&self, x: u8, y: u8) -> i64 {
        self.field[y as usize][x as usize].into()
    }

    pub fn set_point(&mut self, x: u8, y: u8, v: u8) {
        self.field[y as usize][x as usize] = v;
    }

    pub fn push(&mut self, v: i64) {
        self.stack.push(v);
    }

    pub fn pop(&mut self) -> i64 {
        self.stack.pop().unwrap_or_else(|| {
            self.stack.push(0);
            0
        })
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
