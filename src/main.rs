mod board;

use board::Board;

fn main() {
    let (cols, rows) = term_size::dimensions().unwrap();
    let mut board = Board::new(cols, rows);

    loop {
        board.display();
        board.update();
        std::thread::sleep(std::time::Duration::from_millis(200));
        clear_terminal();
    }
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
