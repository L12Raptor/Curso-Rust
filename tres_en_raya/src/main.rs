use std::io;

use rand::Rng;

fn main() {
    main_menu();

    println!("Hasta la próxima :D");
}

fn main_menu() {
    let stdin = io::stdin();

    let mut text: String;

    loop {
        println!("Selecciona una opción:");
        println!("1 - Jugar tres en raya");
        println!("0 - Salir");

        text = String::new();

        let result = stdin.read_line(&mut text);

        match result {
            Ok(_) => {
                let test = &text.trim().parse::<i32>();
                match test {
                    Ok(number) => {
                        if *number == 1 {
                            play_game();
                        }
                        else if *number == 0 {
                            break;
                        }
                    }
                    Err(_) => println!("Debes escribir un número!"),
                }
            }
            Err(_) => {
                println!("Hubo un problema leyendo los caracteres introducidos por teclado!")
            }
        };
    }
}

fn play_game() {
    let stdin = io::stdin();

    let mut board: Board = Board::new();

    let mut text: String;

    let mut row: i32;

    let mut column: i32;

    let mut ok: bool = false;

    while !ok {
        while !ok {
            loop {
                text = String::new();

                println!("Introduce el número de fila que desees ocupar:");

                let result = stdin.read_line(&mut text);

                match result {
                    Ok(_) => {
                        let test = &text.trim().parse::<i32>();
                        match test {
                            Ok(number) => {
                                row = *number;

                                break;
                            }
                            Err(_) => println!("Debes escribir un número!"),
                        }
                    }
                    Err(_) => {
                        println!(
                            "Hubo un problema leyendo los caracteres introducidos por teclado!"
                        )
                    }
                };
            }

            loop {
                text = String::new();

                println!("Introduce el número de columna que desees ocupar:");

                let result = stdin.read_line(&mut text);

                match result {
                    Ok(_) => {
                        let test = &text.trim().parse::<i32>();
                        match test {
                            Ok(number) => {
                                column = *number;

                                break;
                            }
                            Err(_) => println!("Debes escribir un número!"),
                        }
                    }
                    Err(_) => {
                        println!(
                            "Hubo un problema leyendo los caracteres introducidos por teclado!"
                        )
                    }
                };
            }

            if is_valid_board_square(row, column, &board) {
                board.squares[row as usize][column as usize] = 'x';

                ok = true;
            } else {
                println!("La casilla seleccionada no es válida!");

                ok = false;
            }
        }

        ok = false;

        while !ok {
            let mut rng = rand::thread_rng();

            row = rng.gen_range(0..3);
            column = rng.gen_range(0..3);

            if is_valid_board_square(row, column, &board) {
                board.squares[row as usize][column as usize] = 'o';

                ok = true;
            } else {
                ok = false;
            }
        }

        ok = false;

        for line in board.squares {
            for value in line {
                print!(" {} ", value);
            }
            println!();
        }

        if board.squares[0][0] == board.squares[0][1] && board.squares[0][1] == board.squares[0][2]
        {
            if board.squares[0][0] == 'x' {
                println!("Enhorabuena jugador, has ganado!");

                ok = true;
            } else if board.squares[0][0] == 'o' {
                println!("Victoria para la máquina, lo siento jugador, otra vez será.");

                ok = true;
            }
        } else if board.squares[1][0] == board.squares[1][1]
            && board.squares[1][1] == board.squares[1][2]
        {
            if board.squares[1][0] == 'x' {
                println!("Enhorabuena jugador, has ganado!");

                ok = true;
            } else if board.squares[1][0] == 'o' {
                println!("Victoria para la máquina, lo siento jugador, otra vez será.");

                ok = true;
            }
        } else if board.squares[2][0] == board.squares[2][1]
            && board.squares[2][1] == board.squares[2][2]
        {
            if board.squares[2][0] == 'x' {
                println!("Enhorabuena jugador, has ganado!");

                ok = true;
            } else if board.squares[2][0] == 'o' {
                println!("Victoria para la máquina, lo siento jugador, otra vez será.");

                ok = true;
            }
        } else if board.squares[0][0] == board.squares[1][0]
            && board.squares[1][0] == board.squares[2][0]
        {
            if board.squares[0][0] == 'x' {
                println!("Enhorabuena jugador, has ganado!");

                ok = true;
            } else if board.squares[0][0] == 'o' {
                println!("Victoria para la máquina, lo siento jugador, otra vez será.");

                ok = true;
            }
        } else if board.squares[0][1] == board.squares[1][1]
            && board.squares[1][1] == board.squares[2][1]
        {
            if board.squares[0][1] == 'x' {
                println!("Enhorabuena jugador, has ganado!");

                ok = true;
            } else if board.squares[0][1] == 'o' {
                println!("Victoria para la máquina, lo siento jugador, otra vez será.");

                ok = true;
            }
        } else if board.squares[0][2] == board.squares[1][2]
            && board.squares[1][2] == board.squares[2][2]
        {
            if board.squares[0][2] == 'x' {
                println!("Enhorabuena jugador, has ganado!");

                ok = true;
            } else if board.squares[0][2] == 'o' {
                println!("Victoria para la máquina, lo siento jugador, otra vez será.");

                ok = true;
            }
        } else if board.squares[0][0] == board.squares[1][1]
            && board.squares[1][1] == board.squares[2][2]
        {
            if board.squares[0][0] == 'x' {
                println!("Enhorabuena jugador, has ganado!");

                ok = true;
            } else if board.squares[0][0] == 'o' {
                println!("Victoria para la máquina, lo siento jugador, otra vez será.");

                ok = true;
            }
        } else if board.squares[0][2] == board.squares[1][1]
            && board.squares[1][1] == board.squares[2][0]
        {
            if board.squares[0][2] == 'x' {
                println!("Enhorabuena jugador, has ganado!");

                ok = true;
            } else if board.squares[0][2] == 'o' {
                println!("Victoria para la máquina, lo siento jugador, otra vez será.");

                ok = true;
            }
        }
    }
}

struct Board {
    squares: [[char; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            squares: [['_', '_', '_'], ['_', '_', '_'], ['_', '_', '_']],
        }
    }
}

fn is_valid_board_square(row: i32, column: i32, board: &Board) -> bool {
    let mut result: bool = true;

    if !(0..=2).contains(&row)
        || !(0..=2).contains(&column)
        || board.squares[row as usize][column as usize] != '_'
    {
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
fn test_select_already_selected_board_square_should_return_false() {
    let row: i32 = 1;
    let column: i32 = 1;

    let mut board: Board = Board::new();

    board.squares[row as usize][column as usize] = 'o';

    let result: bool = is_valid_board_square(row, column, &board);

    assert!(!result);
}
