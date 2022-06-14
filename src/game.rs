use rand::Rng;

pub struct Field {
    cells: std::cell::RefCell<Vec<u8>>,
    pub size: usize,
}


fn neighbors(x: u8, y: u8, size: usize) -> impl Iterator<Item = (usize, usize)> {
    [(-1, -1),(-1, 0),(-1, 1),(0, -1),(0, 1),(1, -1),(1, 0),(1, 1)]
    .into_iter()
    .filter_map(move |(dx, dy)| {
        let max = (size - 1) as i32;
        let x = x as i32 + dx;
        let y = y as i32 + dy;
        if x < 0 || x > max || y < 0 || y > max {
            return None;
        }
        Some((x as usize, y as usize))
    })
}

impl<'a> Field {
    pub fn new(size: usize) -> Field {
        let mut rng = rand::thread_rng();
        let len = size * size;
        let field = Field {
            cells: std::cell::RefCell::new(vec![0; len]),
            size,
        };
        let mut num_mines = 32;
        {
            let mut cells = field.cells.borrow_mut();
            while num_mines > 0 {
                let ix = rng.gen_range(0..len);
                if cells[ix] == 1 {
                    continue;
                }
                cells[ix] = 1;
                num_mines -= 1;
            }
        }
        field
    }

    fn is_set(&self, x: usize, y: usize, nth_bit: usize) -> bool {
        //dbg!(x, y);
        (self.cells.borrow()[y * self.size + x] >> nth_bit & 1) != 0
    }

    pub fn has_mine(&self, x: usize, y: usize) -> bool {
        self.is_set(x, y, 0)
    }
    pub fn is_swept(&self, x: usize, y: usize) -> bool {
        self.is_set(x, y, 1)
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> bool {
        if self.is_swept(x, y) {
            return false;
        }
        let flag_bit = 0b10;
        let ix = y * self.size + x;
        let mut cells = self.cells.borrow_mut();
        cells[ix] ^= flag_bit;
        cells[ix] & flag_bit != 0
    }

    pub fn sweep(&'a self, x: usize, y: usize) -> Option<Box<dyn Iterator<Item = Sweep> + 'a>> {
        if self.has_mine(x, y) {
            None
        } else if self.is_swept(x, y) || self.mines_nearby(x as u8, y as u8) > 0 {
            Some(Box::new(std::iter::once(Sweep { x: x as u8, y: y as u8, mines_nearby: self.mines_nearby(x as u8, y as u8) })))
        } else {
            Some(Box::new(SweepIterator::new(x as u8, y as u8, self)))
        }
    }

    fn mines_nearby(&self, x: u8, y: u8) -> u8 {
        neighbors(x, y, self.size).fold(0,
                |acc, (x, y)| {
                    if self.has_mine(x, y) {
                        acc + 1
                    } else {
                        acc
                    }
                },
            )
    }
}

pub struct SweepIterator<'a> {
    field: &'a Field,
    inner: Option<Box<SweepIterator<'a>>>,
    index: u8,
    x: u8,
    y: u8
}

const COORDS: [(i32, i32); 9] = [(0, 0), (-1, -1),(-1, 0),(-1, 1),(0, -1),(0, 1),(1, -1),(1, 0),(1, 1)];

impl<'a> SweepIterator<'a> {
    pub fn new(x: u8, y: u8, field: &Field) -> SweepIterator {
        SweepIterator { field, x, y, index: 0, inner: None }
    }
}

impl<'a> Iterator for SweepIterator<'a> {
    type Item = Sweep;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.field.size;
        if let Some(inner) = self.inner.as_mut() {
            if let Some(sweep) = inner.next() {
                return Some(sweep);
            }
        }
        let len = COORDS.len() as u8;
        let max = (size - 1) as i32;
        while self.index < len {
            let (dx, dy) = COORDS[self.index as usize];
            let x = self.x as i32 + dx;
            let y = self.y as i32 + dy;
            self.index += 1;
            if x < 0 || x > max || y < 0 || y > max {
                continue;
            }
            let x = x as u8;
            let y = y as u8;
            if self.field.has_mine(x as usize, y as usize) {
                continue;
            }
            let is_swept = self.field.is_swept(x as usize, y as usize);
            if !is_swept {
                let ix = y as usize * self.field.size + x as usize;
                let mut cells = self.field.cells.borrow_mut();
                cells[ix] |= 1 << 1;
            }
            let mines_nearby = self.field.mines_nearby(x, y);
            if is_swept || mines_nearby > 0 {
                return Some(Sweep{x, y, mines_nearby});
            }
            let mut iter = SweepIterator::new(x, y, self.field);
            let result = iter.next();
            self.inner = Some(Box::new(iter));
            return result;
        }        
        return None;
    }
}

#[derive(Clone)]
pub struct Sweep {
    pub x: u8,
    pub y: u8,
    pub mines_nearby: u8,
}
