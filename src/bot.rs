use crate::board;
use std::cmp;

pub struct Bot {
    board: board::Board2d,
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            board: [['#'; 3]; 3],
        }
    }

    pub fn from(template_board: board::Board2d) -> Bot {
        Bot {
            board: template_board,
        }
    }
}

impl Bot {
    fn retrieve_empty_tiles(&self) -> Vec<board::Coordinates> {
        let mut empty_tiles: Vec<board::Coordinates> = Vec::new();
        for y in 0..3 {
            for (x, tile) in self.board[y].iter().enumerate() {
                if *tile == '#' { empty_tiles.push((y as usize, x as usize)) };
            }
        }
        empty_tiles 
    }

    fn minimax(&self, board: &mut board::Board2d, is_maximizing: bool, depth: i32) -> i32 {
        let state: Option<i32> = match board::Board::from(*board).status() {
            board::BoardStatus::Win => Some(100 - depth), 
            board::BoardStatus::Lose => Some(-100 + depth),
            board::BoardStatus::Draw => Some(0), 
            board::BoardStatus::Undeclared => None
        };

        if let Some(s) = &state {
            println!("{:?}", s);
            return *s;
        };


        let mut best_val = if is_maximizing { -100 } else { 100 };
        let player = if is_maximizing { board::GamePiece::O } else { board::GamePiece::X };
        
        for (x, y) in self.retrieve_empty_tiles() {
            board[y][x] = player.get_char();
            println!("{:?}", board);
            let value = self.minimax(board, !is_maximizing, depth+1);
            best_val = if is_maximizing { cmp::max(value, best_val) } else { cmp::min(value, best_val) };
            //board[y][x] = '#';
        }

        best_val
    }

    pub fn find_best_move(&mut self) -> board::Coordinates {
        let mut coords: board::Coordinates = (0, 0);
        let mut best_val = -100;

        for (x, y) in self.retrieve_empty_tiles() {
            self.board[y][x] = 'O';
            let value = self.minimax(&mut Box::new(self.board), false, 0);
            println!("{}", value);
            self.board[y][x] = '#';
            if best_val < value {
                coords = (x, y);
                best_val = value;
            }
        }

        coords
    }

}
