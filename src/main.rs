use std::{io, str::FromStr};

use valterm_chess::{moves::MoveType, Game, GameStatus, PieceType, Position};

fn main() {
    let mut game = Game::new();
    game.default_board();
    loop {
        match game.update_game() {
            GameStatus::Check(color) => println!("{:?} is checked!", color),
            GameStatus::Checkmate(color) => println!("{:?} is checkmated!", color),
            GameStatus::Stalemate => println!("Stalemate! :("),
            GameStatus::Promotion(_) => {
                println!("Promotion (auto queen :sunglasses:)");
                game.promote(PieceType::Queen);
                continue;
            }
            _ => (),
        };

        game.print_board();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let from = Position::from_str(iter.next().unwrap().trim()).unwrap();
        let to = Position::from_str(iter.next().unwrap().trim()).unwrap();

        if game.move_piece(from, to) == MoveType::Invalid {
            println!("Invalid move!");
        }
    }
}