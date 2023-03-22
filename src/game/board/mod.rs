pub type BoardRow = Vec<bool>;
pub type Board = Vec<Vec<bool>>;

pub type VisualBoardRow = Vec<char>;
pub type VisualBoard = Vec<Vec<char>>;

fn get_random_bool() -> bool {
    return rand::random::<bool>();
}

fn get_random_board_row(size: u8) -> Vec<bool> {
    let row: Vec<bool> = (0..size)
        .map(|_| get_random_bool())
        .collect();

    return row;
}

pub fn get_random_board(rows: u8, columns: u8) -> Board {
    let board: Vec<Vec<bool>> = (0..rows)
        .map(|_| get_random_board_row(columns))
        .collect();

    return board;
}
