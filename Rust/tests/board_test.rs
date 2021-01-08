#[path = "../src/board.rs"]
pub mod board;

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
}
