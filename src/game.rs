use rand::Rng;

pub struct Field {
    cells: std::cell::RefCell<Vec<u8>>,
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

impl<'a> Field {
    pub fn new(size: usize) -> Field {
        let mut rng = rand::thread_rng();
        let len = size * size;
        let field = Field {cells: RefCell::new(vec![0; len]), size};
        let mut num_mines = 32;
        {
            let mut cells = field.cells.borrow_mut();
            while num_mines > 0 {
                let ix = rng.gen_range(0..len);
                if cells[ix] == 1 { continue; }
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
        self.is_set(x,y, 0)
    }
    pub fn is_swept(&self, x: usize, y: usize) -> bool {
        self.is_set(x, y, 1)
    }    

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> bool {
        if self.is_swept(x, y) { return false; }
        let flag_bit = 0b10;
        let ix = y * self.size + x;
        let mut cells = self.cells.borrow_mut();
        cells[ix] ^= flag_bit;
        cells[ix] & flag_bit != 0
    }

    pub fn sweep(&'a self, x: usize, y: usize) -> Option<Box<dyn Iterator<Item = Sweep> + 'a>> {       
        if self.has_mine(x, y) {
            None
        } else {
            Some(self.sweep_rec(x, y))
        }
    }

    fn sweep_rec(&'a self, x: usize, y: usize) -> Box<dyn Iterator<Item = Sweep> + 'a> {
        let current = Sweep{ x, y, mines_nearby: self.mines_nearby(x, y) };
        if self.is_swept(x, y) || current.mines_nearby > 0 { return Box::new(std::iter::once(current)); }
        let size = self.size;
        {
            let mut cells = self.cells.borrow_mut();
            cells[y * self.size + x] |= 1 << 1;
        }        
        Box::new(std::iter::once(current).chain(neighbors(x, y, size).flat_map(|(x, y)| {            
            self.sweep_rec(x, y)
        })))
    }

    fn mines_nearby(&self, x: usize, y: usize) -> usize {
        neighbors(x, y, self.size).fold(0, | acc, (x, y)| {
            if self.has_mine(x, y) { acc + 1 } else { acc }
        })
    }
    
}

pub struct Sweep {
    pub x: usize,
    pub y: usize,
    pub mines_nearby: usize
}