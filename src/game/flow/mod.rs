use itertools::Itertools;
use crate::game::board::{
    Board,
};


struct Coords {
    row: i64,
    col: i64,
}

pub fn generate_next_board_iteration(board: Board) -> Board {
    let mut new_board = board.clone();

    for (i, row) in board.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            let coords = Coords { row: (i as i64), col: (j as i64) };
            let new_cell_value = will_be_alive(coords, board.clone());
            new_board[i][j] = new_cell_value;
        }
    }

    return new_board;
}


fn will_be_alive(cell_coords: Coords, board: Board) -> bool {
    let mut count = 0;
    let current_cell_status = board[(cell_coords.row as usize)][(cell_coords.col as usize)];

    for row in -1..=1 {
        for col in -1..=1 {
            if row != 0 || col != 0 {
                let coords_to_check = Coords {
                    row: cell_coords.row + row,
                    col: cell_coords.col + col,
                };
                let cell_status = get_cell_value(coords_to_check, board.clone());

                if cell_status {
                    count += 1;
                }
            }
        }
    }

    if current_cell_status {
        return count == 3 || count == 2;
    } else {
        return count == 3;
    }
}

fn get_cell_value(cell_coords: Coords, board: Board) -> bool {
    if cell_coords.row < 0
        || cell_coords.col < 0
        || cell_coords.row >= (board.len() as i64)
        || cell_coords.col >= (board[0].len() as i64) {
        return false;
    }

    return board[cell_coords.row as usize][cell_coords.col as usize];
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_will_be_alive() {
        // given
        let mock_board: Board = vec![
            vec![false, true, false],
            vec![true, false, false],
            vec![true, false, false],
        ];
        let coords_to_check = Coords { row: 1, col: 1 };

        // when
        let res = will_be_alive(coords_to_check, mock_board);

        assert_eq!(res, true)
    }

    #[test]
    fn test_get_cell_value_outside_range_1() {
        // given
        let mock_board: Board = vec![
            vec![false, true, false]
        ];
        let coords_to_check = Coords { row: -1, col: -1 };

        // when
        let res = get_cell_value(coords_to_check, mock_board);

        assert_eq!(res, false)
    }

    #[test]
    fn test_get_cell_value_outside_range_2() {
        // given
        let mock_board: Board = vec![
            vec![false, true, false]
        ];
        let coords_to_check = Coords { row: 1, col: 3 };

        // when
        let res = get_cell_value(coords_to_check, mock_board);

        assert_eq!(res, false)
    }

    #[test]
    fn test_get_cell_value_outside_range_3() {
        // given
        let mock_board: Board = vec![
            vec![false, true, false]
        ];
        let coords_to_check = Coords { row: 0, col: 3 };

        // when
        let res = get_cell_value(coords_to_check, mock_board);

        assert_eq!(res, false)
    }

    #[test]
    fn test_get_cell_in_range() {
        // given
        let mock_board: Board = vec![
            vec![false, true, false],
            vec![true, false, false],
            vec![true, false, false],
        ];
        let coords_to_check = Coords { row: 0, col: 1 };

        // when
        let res = get_cell_value(coords_to_check, mock_board);

        // then
        assert_eq!(res, true)
    }

    // #[ignore]
    #[test]
    fn test_generate_next_board_iteration() {
        // given
        let mock_board: Board = vec![
            vec![false, true, false],
            vec![true, false, false],
            vec![true, false, false],
            vec![false, false, false],
        ];

        let expected_result: Board = vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
            vec![false, false, false],
        ];

        // when
        let res = generate_next_board_iteration(mock_board);

        // then
        assert_eq!(res, expected_result)
    }
}
