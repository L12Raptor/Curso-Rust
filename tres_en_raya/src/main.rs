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

fn select_board_square(row: i32, column: i32) -> bool {
    let mut result: bool = true;

    if !(0..=2).contains(&row) || !(0..=2).contains(&column) {
        result = false;
    }

    result
}

#[test]
fn test_select_valid_board_square_should_return_true() {
    let row: i32 = 2;
    let column: i32 = 0;

    let result: bool = select_board_square(row, column);

    assert!(result);
}

#[test]
fn test_select_row_less_than_zero_should_return_false() {
    let row: i32 = -1;
    let column: i32 = 0;

    let result: bool = select_board_square(row, column);

    assert!(!result);
}

#[test]
fn test_select_column_less_than_zero_should_return_false() {
    let row: i32 = 0;
    let column: i32 = -1;

    let result: bool = select_board_square(row, column);

    assert!(!result);
}

#[test]
fn test_select_row_greater_than_two_should_return_false() {
    let row: i32 = 3;
    let column: i32 = 0;

    let result: bool = select_board_square(row, column);

    assert!(!result);
}

#[test]
fn test_select_column_greater_than_two_should_return_false() {
    let row: i32 = 0;
    let column: i32 = 3;

    let result: bool = select_board_square(row, column);

    assert!(!result);
}

#[test]
fn test_select_already_selected_board_square_should_return_false()
{
    let row: i32 = 1;
    let column: i32 = 1;

    select_board_square(row, column);

    let result: bool = select_board_square(row, column);

    assert!(!result);
}
