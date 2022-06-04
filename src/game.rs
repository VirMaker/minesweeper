use rand::Rng;
use std::iter;

pub struct Field {
    cells: Vec<u8>,
    pub size: usize,
}

fn neighbors(x: usize, y: usize, size: usize) -> impl Iterator<Item = (usize, usize)> {        
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1),  (1, 0),  (1, 1)]
     .into_iter().filter_map(move |(dx, dy)| {
        let max = (size - 1) as i32;
        let x = x as i32 + dx;
        let y = y as i32 + dy;
        if x < 0 || x > max || y < 0 || y > max {
            return None;
        } 
        Some((x as usize, y as usize))
    })
}

impl Field {
    pub fn new(size: usize) -> Field {
        let mut rng = rand::thread_rng();
        let len = size * size;
        let mut field = Field {cells: vec![0; len], size};
        let mut num_mines = 32;
        while num_mines > 0 {
            let ix = rng.gen_range(0..len);
            if field.cells[ix] == 1 { continue; }
            field.cells[ix] = 1;
            num_mines -= 1;
        }
        field
    }

    fn is_set(&self, x: usize, y: usize, nth_bit: usize) -> bool {
        //dbg!(x, y);
        (self.cells[y * self.size + x] >> nth_bit & 1) != 0
    }

    fn as_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.cells[y * self.size + x]
    }

    pub fn has_mine(&self, x: usize, y: usize) -> bool {
        self.is_set(x,y, 0)
    }
    pub fn is_swept(&self, x: usize, y: usize) -> bool {
        self.is_set(x, y, 1)
    }
    pub fn is_flagged(&self, x: usize, y: usize) -> bool {
        self.is_set(x,y, 2)
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> bool {
        if self.is_swept(x, y) { return false; }
        *self.as_mut(x, y) ^= 1 << 2;
        self.is_flagged(x, y)
    }

    pub fn sweep(&mut self, x: usize, y: usize) -> Option<Vec<Sweep>> {       
        if self.has_mine(x, y) {
            None
        } else {
            let current = Sweep{ x, y, mines_nearby: self.mines_nearby(x, y) };
            if self.is_swept(x, y) || current.mines_nearby > 0  {
                Some(vec![current])
            } else {
                *self.as_mut(x, y) |= 1 << 1;
                let mut result = vec![current];
                for (x, y) in neighbors(x, y, self.size) {
                    result.append(&mut self.sweep(x, y).unwrap());
                };
                Some(result)
            }
        }
    }

    fn mines_nearby(&self, x: usize, y: usize) -> usize {
        neighbors(x, y, self.size).fold(0, |mut acc, (x, y)| {
            if self.has_mine(x, y) {
                acc += 1;
            }
            acc
        })
    }
    
}

pub struct Sweep {
    pub x: usize,
    pub y: usize,
    pub mines_nearby: usize
}