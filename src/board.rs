pub type Coordinates = (usize, usize);
pub type Board2d = [[char; 3]; 3];

#[derive(PartialEq)]
pub enum BoardStatus {
    Win,
    Lose,
    Draw,
    Undeclared
}

pub enum GamePiece {
    X,
    O
}

impl GamePiece {
    pub fn get_char(&self) -> char {
        match self {
            GamePiece::X => return 'X',
            GamePiece::O => return 'O',
        };
    }
}

pub struct Board {
    board: Board2d,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [['#'; 3]; 3],
        }
    }
    
    pub fn from(template_board: Board2d) -> Board {
        Board {
            board: template_board,
        }
    }

    pub fn all_combinations(&self) -> Vec<[char; 3]> {
        let mut combinations: Vec<[char; 3]> = Vec::new();
        let mut vertical: [char; 3] = ['#'; 3];

        for row in self.board.iter() {
            combinations.push(row.to_owned());
        }

        for i in 0..=2 {
            for j in 0..=2 {
                vertical[j] = self.board[j][i];
            }

            combinations.push(vertical);
        }

        for i in 0..=2 {
            vertical[i] = self.board[i][i];
        }
        combinations.push(vertical);

        for i in 0..=2 {
            vertical[i] = self.board[i][2 - i];
        }
        combinations.push(vertical);

        combinations
    }

    fn get_winner(combinations: [char; 3], game_piece: GamePiece) -> BoardStatus {
        let game_piece = game_piece.get_char();
        let filtered_combination: Vec<&char> = combinations
            .iter()
            .filter(|comb| **comb == game_piece)
            .collect();

        if filtered_combination.len() == 3 {
            return BoardStatus::Win
        }
        
        BoardStatus::Undeclared
    }

    pub fn status(&self) -> BoardStatus {
        let combinations = self.all_combinations();
        let mut full_board = true;

        for comb in combinations {
            if Board::get_winner(comb, GamePiece::X) == BoardStatus::Win { return BoardStatus::Win }
            if Board::get_winner(comb, GamePiece::O) == BoardStatus::Win { return BoardStatus::Lose }
        
            for tile in comb.iter() {
                if *tile == '#' { full_board = false }
            }      
        }

        if full_board { return BoardStatus::Draw }

        BoardStatus::Undeclared
    }

    pub fn place_game_piece(&mut self, coords: Coordinates, game_piece: GamePiece){
        self.board[coords.1][coords.0] = game_piece.get_char();
    }

    pub fn get_board(&mut self) -> Board2d {
        return self.board; 
    }
}
