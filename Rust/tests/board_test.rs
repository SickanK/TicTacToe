#[path = "../src/board.rs"]
pub mod board;
#[path = "../src/bot.rs"]
pub mod bot;

#[cfg(test)]
mod board_test {
    use super::*;

    #[test]
    fn result_of_game() {
        let mut board = board::Board::new();

        board.place_game_piece((0, 0), board::GamePiece::X);
        board.place_game_piece((0, 1), board::GamePiece::X);
        board.place_game_piece((0, 2), board::GamePiece::X);

        let result: board::BoardStatus = board.status();
        let is_win = result == board::BoardStatus::Win;
        assert_eq!(true, is_win);
    }

    #[test]
    fn bot_placement() {
        let mut board = board::Board::new();

        board.place_game_piece((1, 1), board::GamePiece::X);
        
        let botmove = bot::Bot::from(board.get_board()).find_best_move();
        board.place_game_piece(botmove, board::GamePiece::O).unwrap();

        println!("{:?}", board.get_board());

        assert_eq!('O', board.get_board()[0][0]);
    }


}
