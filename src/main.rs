use std::io::stdin;

fn main() {
    let mut board = GameBoard::new(Player::X);
    while !board.game_over {
        println!("{}", board.to_string());
        println!("It's {}'s turn. Where do you want to go to?:", {
            match board.turn {
                Player::X => "X",
                Player::O => "O"
            }
        });
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_error) => {
                println!("Problem getting input!");
                continue;
            }
        }
        let pos = match input.trim().parse::<usize>() {
            Ok(num) => {num},
            Err(_error) => {
                println!("Please put in a number!");
                continue;
            }
        };

        if !board.verify_move(pos-1) {
            println!("Invalid move!");
            continue;
        }

        board.play(pos-1);

        if board.game_over {
            println!("The game is over! {} won!", match board.winner{
                None => "No one",
                Some(player) => match player {
                    Player::X => "X",
                    Player::O => "O"
                }

            });
            println!("{}", board.to_string());
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
enum Player {
    X, O
}

struct GameBoard {
    turn: Player,
    board: [Option<Player>; 9],
    winner: Option<Player>,
    game_over: bool
}

impl GameBoard {
    fn new(first: Player) -> GameBoard {
        return GameBoard {
            turn: first,
            board: [None; 9],
            winner: None,
            game_over: false
        }
    }

    fn verify_move(&self, pos: usize) -> bool {
        if pos > self.board.len() {
            return false;
        }
        if self.board[pos] != None {
            return false;
        }
        if self.game_over {
            return false;
        }

        true
    }

    fn play(&mut self, pos: usize) {
        self.board[pos] = Some(self.turn);

        match self.turn {
            Player::X => self.turn = Player::O,
            Player::O => self.turn = Player::X
        }

        if self.player_won(Player::X) {
            self.game_over = true;
            self.winner = Some(Player::X)
        } else if self.player_won(Player::O) {
            self.game_over = true;
            self.winner = Some(Player::O)
        } else {
            let mut empty_pos = false;
            for pos in 0..9 {
                if self.board[pos] == None {
                    empty_pos = true;
                    break;
                }
            }
            if !empty_pos {
                self.game_over = true;
            }
        }
    }

    fn player_won(&self, player: Player) -> bool {
        for row in 0..3 {
            let start = row * 3;
            if Some(player) == self.board[start] && Some(player) == self.board[start+1] && Some(player) == self.board[start+2] {
                return true;
            }
        }

        for col in 0..3 {
            if Some(player) == self.board[col] && Some(player) == self.board[col+3] && Some(player) == self.board[col+6] {
                return true;
            }
        }

        if Some(player) == self.board[0] && Some(player) == self.board[4] && Some(player) == self.board[8] {
            return true;
        }

        if Some(player) == self.board[2] && Some(player) == self.board[4] && Some(player) == self.board[6] {
            return true;
        }

        return false;
    }

    fn to_string(&self) -> String {
        let mut text = String::new();
        for i in 0..self.board.len() {
            match self.board[i] {
                Some(Player::X) => text.push('X'),
                Some(Player::O) => text.push('O'),
                None => text.push((i + 1).to_string().chars().next().unwrap_or('-'))
            }

            if i % 3 == 2 {
                text.push('\n');
            } else {
                text.push(' ')
            }
        }

        return text
    }
}