use std::{io::{Write, self}};
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
    }
    println!("");
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


fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let mut board = board::Board::new();
    board.all_combinations();
    board.status();

    /*
     * 1. Show board
     * 2. Ask for row
     * 3. Show which row was asked and ask for tile
     * 4. Place tile and show board
     * 5. Sleep for somewhere between 1-3 seconds and display opponent is placing
     * 6. Show bot placement
     * 7. goto step 1
     * */

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
        let row: u32 = row.trim().parse().unwrap();

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        println!("Board:");
        print_board_highlight(&board.get_board(), (row-1) as u8);

        println!("\n-----");
        print_board_row(&board.get_board(), (row-1) as u8);
        println!("\n-----");

        let mut tile = String::new();
        println!("\nChoose a tile (1-3): ");
        print!("Tile: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut tile).unwrap();
        let tile: u32 = tile.trim().parse().unwrap();

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        board.place_game_piece(
            ((tile-1) as usize, (row-1) as usize), 
            board::GamePiece::X
        );

        let botmove = bot::Bot::from(board.get_board()).find_best_move();
        board.place_game_piece(botmove, board::GamePiece::O);


    }
}


