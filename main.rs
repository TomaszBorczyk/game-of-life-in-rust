use std::{thread, time};

type BoardRow = Vec<bool>;
type Board = Vec<Vec<bool>>;

type VisualBoardRow = Vec<char>;
type VisualBoard = Vec<Vec<char>>;

fn get_random_bool() -> bool {
    return rand::random::<bool>()
}

fn get_random_board_row(size: u8) -> Vec<bool> {
    let row: Vec<bool> = (0..size)
        .map(|_| get_random_bool())
        .collect();

    return row;
}

fn get_random_board(x_size: u8, y_size: u8) -> Board {
    let board: Vec<Vec<bool>> = (0..y_size)
        .map(|_| get_random_board_row(x_size))
        .collect();

    return board;
}

fn represent_cell_visually(cell: bool) -> char {
    return if cell {
        '*'
    } else {
        ' '
    }
}

fn represent_board_row_visually(row: BoardRow) -> VisualBoardRow {
    let visual_row: Vec<char> = row
        .iter()
        .map(|v| represent_cell_visually(*v))
        .collect();
    return visual_row;
}

fn represent_board_visually(board: Board) -> VisualBoard {
    return board
        .iter()
        .map(|row| represent_board_row_visually(row.clone()))
        .collect();
}

fn print_board(board: VisualBoard) {
    board
        .iter()
        .for_each(|row| println!("{:?}", row));
}

fn sleep(miliseconds: u64) {
    let duration = time::Duration::from_millis(miliseconds);
    thread::sleep(duration);
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    clear_terminal();

    loop {
        let board = get_random_board(3, 4);
        let visual_board = represent_board_visually(board);
        print_board(visual_board);

        sleep(1000);
        clear_terminal();
    }
}
