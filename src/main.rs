use rand::{thread_rng, Rng};

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn init_state(width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut state = vec![vec![true; width]; height];
    let mut rng = thread_rng();
    for i in 0..height {
        for j in 0..width {
            state[i][j] = rng.gen_bool(0.2);
        }
    }
    return state;
}

fn update_state(state: &mut Vec<Vec<bool>>) {
    let rows = state.len();
    let cols = state[0].len();
    let mut updated_state = vec![vec![true; cols]; rows];
    let mut rng = thread_rng();

    // Create the updated state
    for i in 0..state.len() {
        for j in 0..state[i].len() {
            updated_state[i][j] = rng.gen_bool(0.2);
        }
    }

    // Replace borrowed state with the updated state
    *state = updated_state;
}

fn print_state(state: &Vec<Vec<bool>>) {
    for i in 0..state.len() {
        for j in 0..state[i].len() {
            match state[i][j] {
                true => print!("■"), // ■, █
                false => print!(" "),
            }
        }
        println!();
    }
}

fn main() {
    let (width, height) = term_size::dimensions().unwrap();
    let mut state = init_state(width, height);

    loop {
        clear();
        print_state(&state);
        update_state(&mut state);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
