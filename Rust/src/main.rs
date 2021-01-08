extern crate rand;
use std::{time, io::{Write, self}};
use rand::Rng;
mod board;
mod bot;

fn print_board(&board: &board::Board2d){
    for b in &board {
        for (i, a) in b.iter().enumerate() {
            if i == 2 {
                print!("{}", *a);
            } else {
                print!("{},", *a);
            }
        }
        println!("");
    }
}

fn print_board_highlight(&board: &board::Board2d, highlight: u8){
    for (i, b) in board.iter().enumerate() {
        for (j, a) in b.iter().enumerate() {
            if j == 2 {
                print!("{}", *a);
            } else {
                print!("{}," , *a);
            }
        }
        if i == highlight as usize { print!(" <---") }
        println!("");
    }
}

fn print_board_row(&board: &board::Board2d, row: u8){
    for (i, a) in board[row as usize].iter().enumerate() {
        if i == 2 {
            print!("{}", *a);
        } else {
            print!("{},", *a);
        }
    }
}

fn print_end_statement(state: board::BoardStatus){
    if state != board::BoardStatus::Undeclared {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    match state {
        board::BoardStatus::Win => println!("You won!"),
        board::BoardStatus::Lose => println!("You lose."),
        board::BoardStatus::Draw => println!("It's a draw!"),
        board::BoardStatus::Undeclared => print!(""),
    }
    
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let mut board = board::Board::new();
    board.all_combinations();
    board.status();

    println!("Welcome to TicTacToe! (3x3)");
    println!("X: You\nO: Opponent\n#: Empty\n");
    println!("\nPress enter to play!");

    let mut nothing = String::new();
    io::stdin().read_line(&mut nothing).unwrap();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        println!("Board:");
        print_board(&board.get_board());

        let mut row = String::new();
        println!("\nChoose a row (1-3): ");
        print!("Row: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut row).unwrap();
        let row: u32 = match row.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("\n\nError:  Enter a valid number.\n\n");
                continue;
            }
        };

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        if !(0..=2).contains(&(row as u32)) {
            println!("\n\nError: Number must be between between 1 and 3.\n\n");
            continue;
        }

        println!("Board:");
        print_board_highlight(&board.get_board(), (row-1) as u8);
    
        println!("\nChoosen row:");
        print_board_row(&board.get_board(), (row-1) as u8);
        println!("");

        let mut tile = String::new();
        println!("\nChoose a tile (1-3): ");
        print!("Tile: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut tile).unwrap();
        let tile: u32 = match tile.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("\n\nError: Enter a valid number.\n\n");
                continue;
            }
        };
        
        {
            let status = board.status();
            if status != board::BoardStatus::Undeclared {
                print_end_statement(status);
                break;
            }
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        match board.place_game_piece(
            ((tile-1) as usize, (row-1) as usize), 
            board::GamePiece::X
        ) {
            Ok(()) => print!(""),
            Err(msg) => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("\n\nError: {}.\n\n", msg);
                continue;
            }
        };

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Opponent is placing...");


        let sleep_time = rand::thread_rng().gen_range(100..2000);
        let sleep = time::Duration::from_millis(sleep_time);
        std::thread::sleep(sleep);

        let botmove = bot::Bot::from(board.get_board()).find_best_move();
        board.place_game_piece(botmove, board::GamePiece::O).unwrap();

        {
            let status = board.status();
            if status != board::BoardStatus::Undeclared {
                print_end_statement(status);
                break;
            }
        }
    }
}
