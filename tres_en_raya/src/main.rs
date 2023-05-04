fn main() {
    
}

#[test]
fn test_select_valid_board_square_should_return_true()
{
    let row: i32 = 2;
    let column: i32 = 0;

    let result: bool = select_board_square(row, column);

    assert!(result);
}

#[test]
fn test_select_row_less_than_zero_should_return_false()
{
    let row: i32 = -1;
    let column: i32 = 0;

    let result: bool = select_board_square(row, column);

    assert!(!result);
}

fn test_select_column_less_than_zero_should_return_false()
{
    let row: i32 = 0;
    let column: i32 = -1;

    let result: bool = select_board_square(row, column);

    assert!(!result);
}

#[test]
fn test_select_row_greater_than_two_should_return_false()
{
    let row: i32 = 3;
    let column: i32 = 0;

    let result: bool = select_board_square(row, column);

    assert!(!result);
}

#[test]
fn test_select_column_greater_than_two_should_return_false()
{
    let row: i32 = 0;
    let column: i32 = 3;

    let result: bool = select_board_square(row, column);

    assert!(!result);
}
