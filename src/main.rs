fn main() {
    let (w, h) = term_size::dimensions().unwrap();
    let delay = std::time::Duration::from_millis(250);

    let mut character = '█';
    loop {
        for _ in 0..h {
            for _ in 0..w {
                print!("{}", character);
            }
            println!();
        }
        character = if character == '█' { ' ' } else { '█' };

        std::thread::sleep(delay);
        clear_console();
    }
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
