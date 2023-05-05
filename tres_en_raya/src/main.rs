fn main() {

}

struct Board {
    squares: [[char; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board { squares: [['_', '_', '_'], ['_', '_', '_'], ['_', '_', '_']] }
    }
}

fn is_valid_board_square(row: i32, column: i32, board: &Board) -> bool {
    let mut result: bool = true;

    if !(0..=2).contains(&row) || !(0..=2).contains(&column) || board.squares[row as usize][column as usize] != '_' {
        result = false;
    }

    result
}

#[test]
fn test_select_valid_board_square_should_return_true() {
    let row: i32 = 2;
    let column: i32 = 0;

    let board: Board = Board::new();

    let result: bool = is_valid_board_square(row, column, &board);

    assert!(result);
}

#[test]
fn test_select_row_less_than_zero_should_return_false() {
    let row: i32 = -1;
    let column: i32 = 0;

    let board: Board = Board::new();

    let result: bool = is_valid_board_square(row, column, &board);

    assert!(!result);
}

#[test]
fn test_select_column_less_than_zero_should_return_false() {
    let row: i32 = 0;
    let column: i32 = -1;

    let board: Board = Board::new();

    let result: bool = is_valid_board_square(row, column, &board);

    assert!(!result);
}

#[test]
fn test_select_row_greater_than_two_should_return_false() {
    let row: i32 = 3;
    let column: i32 = 0;

    let board: Board = Board::new();

    let result: bool = is_valid_board_square(row, column, &board);

    assert!(!result);
}

#[test]
fn test_select_column_greater_than_two_should_return_false() {
    let row: i32 = 0;
    let column: i32 = 3;

    let board: Board = Board::new();

    let result: bool = is_valid_board_square(row, column, &board);

    assert!(!result);
}

#[test]
fn test_select_already_selected_board_square_should_return_false()
{
    let row: i32 = 1;
    let column: i32 = 1;

    let mut board: Board = Board::new();

    board.squares[row as usize][column as usize] = 'o';

    let result: bool = is_valid_board_square(row, column, &board);

    assert!(!result);
}
