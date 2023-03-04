use rand::{thread_rng, Rng};

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn init_state(cols: usize, rows: usize) -> Vec<Vec<bool>> {
    let mut state = vec![vec![true; cols]; rows];
    let mut rng = thread_rng();
    for i in 0..rows {
        for j in 0..cols {
            state[i][j] = rng.gen_bool(0.2);
        }
    }
    return state;
}

fn update_state(state: &mut Vec<Vec<bool>>) {
    let rows = state.len();
    let cols = state[0].len();
    let mut updated_state = vec![vec![false; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let mut neighbors = 0;
            for k in (i.saturating_sub(1))..=(i + 1).min(rows - 1) {
                for l in (j.saturating_sub(1))..=(j + 1).min(cols - 1) {
                    if (k != i || l != j) && state[k][l] {
                        neighbors += 1;
                    }
                }
            }

            if !state[i][j] {
                if neighbors == 3 {
                    updated_state[i][j] = true;
                }
            } else if neighbors > 1 && neighbors < 4 {
                updated_state[i][j] = true;
            }
        }
    }

    *state = updated_state;
}

fn print_state(state: &Vec<Vec<bool>>) {
    for i in 0..state.len() {
        for j in 0..state[i].len() {
            match state[i][j] {
                true => print!("█"),
                false => print!(" "),
            }
        }
    }
}

fn main() {
    let (cols, rows) = term_size::dimensions().unwrap();
    let mut state = init_state(cols, rows);

    loop {
        clear();
        print_state(&state);
        update_state(&mut state);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
