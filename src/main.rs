use game::Game;
use chess_move::ChessMove;

pub mod piece;
pub mod game;
pub mod board;
pub mod chess_move;

fn main() {
    let mut game = Game::new_standard();
    println!("{}", game);
    let moves = ChessMove::get_possible_moves(&game.board);
    println!("{:?}", moves);
}
