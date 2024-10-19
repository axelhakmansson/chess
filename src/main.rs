use game::Game;
use chess_move::ChessMove;
use piece::Color::{White, Black};

pub mod piece;
pub mod game;
pub mod board;
pub mod chess_move;

fn main() {
    let mut game = Game::new_standard();
    println!("{}", game);
}
