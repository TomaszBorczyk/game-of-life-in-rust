pub mod board;
pub mod display;
pub mod flow;

use std::{thread, time};

use crate::game::flow::{
    generate_next_board_iteration
};
use crate::game::board::{
    get_random_board
};
use crate::game::display::{
    print_board
};


pub fn game_loop(rows: u8, columns: u8) {
    clear_terminal();
    let mut board = get_random_board(rows, columns);

    loop {
        print_board(board.clone());
        board = generate_next_board_iteration(board.clone());
        sleep(50);
        clear_terminal();

    }
}

fn sleep(milliseconds: u64) {
    let duration = time::Duration::from_millis(milliseconds);
    thread::sleep(duration);
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
