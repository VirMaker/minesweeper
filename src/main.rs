mod game;
use console::{Term, Key};
use game::{Sweep};

fn main() {
    let mut x = 1usize;
    let mut y = 2usize;
    let (dx, dy) = (x, y);
    let term = Term::stdout();
    let mut exit = false;
    //move cursor inside the box
    let len = 16usize;
    term.clear_screen().unwrap();
    let mut field = game::Field::new(len);
    println!("Press SPACE to sweep, ENTER to flag, ESC to exit");
    draw_field(len);
    term.move_cursor_to(x, y).unwrap();
    while !exit {
        for ch in term.read_key() {
            match ch {
                Key::ArrowUp => y = if y == dy {dy} else {y - 1},
                Key::ArrowDown => y = if y == len + 1 {len + 1} else {y + 1},
                Key::ArrowLeft => x = if x == dx {dx} else {x - 1},
                Key::ArrowRight => x = if x == len {len} else {x + 1},
                Key::Escape => exit = true,
                Key::Enter => {
                    if field.toggle_flag(x - dx, y - dy) { 
                        print!("F"); 
                    }
                    else if field.is_swept(x - dx, y - dy) {
                        print!(" ");
                    } else {
                        print!("\u{2591}");
                    }
                },
                Key::Char(' ') => {
                    match &mut field.sweep(x - dx, y - dy) {
                        Some(sweeps) => update_field(&term, sweeps),
                        None => end_game(x, y)
                    }
                },
                _ => ()
            }
            term.move_cursor_to(x, y).unwrap();
        }
    }
    term.move_cursor_to(0, len + 3).unwrap();
}

fn draw_field(len: usize) {
    // top row
    print!("\u{250C}");
    for _ in 0..len {
        print!("\u{2500}");
    }
    println!("\u{2510}");
    //middle
    for _ in 0..len {
        print!("\u{2502}");
        for _ in 0..len {
            print!("\u{2591}");
        }
        println!("\u{2502}");
    }
    //bottom row
    print!("\u{2514}");
    for _ in 0..len {
        print!("\u{2500}");
    }
    println!("\u{2518}");
}

fn update_field(term: &Term, sweeps: &[Sweep]) {
    for sweep in sweeps {
        term.move_cursor_to(sweep.x + 1, sweep.y + 2).unwrap();
        match sweep.mines_nearby {
            0 => print!(" "),
            _ => print!("{0}", sweep.mines_nearby)
        }
    }
}

fn end_game(_x: usize, _y : usize) {
    print!("X")
}
