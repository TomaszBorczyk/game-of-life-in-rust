use crate::game::board::{
    Board,
    BoardRow,
    VisualBoardRow,
    VisualBoard
};

pub fn print_board(board: Board) {
    let visual_board = represent_board_visually(board);
    print_visual_board(visual_board);
}

fn print_visual_board(board: VisualBoard) {
    board
        .iter()
        .for_each(|row| print_row(row.clone()));
}

fn print_row(row: VisualBoardRow) {
    let joined = row
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", joined);
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

