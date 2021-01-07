use std::{io};
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


fn main() {
    let mut board = board::Board::new();
    board.all_combinations();
    board.status();

    print_board(&board.get_board());

    loop {
        let mut x = String::new();
        println!("x:");
        io::stdin().read_line(&mut x).unwrap();
        let x: u32 = x.trim().parse().unwrap();
            
        let mut y = String::new();
        println!("y:");
        io::stdin().read_line(&mut y).unwrap();
        let y: u32 = y.trim().parse().unwrap();

        board.place_game_piece(((x-1) as usize, (y-1) as usize), board::GamePiece::X);
        println!("{:?}", board.get_board());
        let botmove = bot::Bot::from(board.get_board()).find_best_move();
        println!("{:?}", botmove); 
        board.place_game_piece(botmove, board::GamePiece::O);
        print_board(&board.get_board());


    }
}


