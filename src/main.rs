use rand::{thread_rng, Rng};

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn init_cells(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut cells = vec![vec![true; width]; height];
    let mut rng = thread_rng();
    for i in 0..height {
        for j in 0..width {
            cells[i][j] = rng.gen_bool(0.3);
        }
    }
    return cells;
}

fn main() {
    let (width, height) = term_size::dimensions().unwrap();
    let cells = init_cells(width, height);

    clear_console();

    for i in 0..height {
        for j in 0..width {
            match cells[i][j] {
                true => print!("█"),
                false => print!(" "),
            }
        }
        println!();
    }
}
