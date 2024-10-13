use crate::board::Board;
use crate::piece::PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
use std::fmt::{self, write, Display};



const BISHOP_DIR: [i32; 4] = [-9, -7, 7, 9];
const ROOK_DIR: [i32; 4] = [-8, -1, 1, 8];
const KNIGHT_DIR: [i32; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
const WHITE_PAWN_DIR: [i32; 4] = [-8, -16, -7, -9];
const BLACK_PAWN_DIR: [i32; 4] = [8, 16, 7, 9];


#[derive(Debug)]
pub struct ChessMove {
    from: usize,
    to: usize
}

impl ChessMove {
    fn new(from: usize, to: usize) -> ChessMove {
        ChessMove {
            from, 
            to
        }
    }

    fn get_legal_moves(board: &Board) -> Vec<ChessMove> {
        let mut possible_moves = ChessMove::get_possible_moves(board);

        let mut legal_moves = Vec::new();
        legal_moves
    }

    pub fn get_possible_moves(board: &Board) -> Vec<ChessMove>{
        let mut possible_moves = Vec::new();

        for index in 0..board.board.len() {
            if let Some(piece) = board.board[index] {
                match piece.p_type {
                    King(_) => {
                        for dir in BISHOP_DIR {
                            let new_index = index as i32 + dir;
                            if index % 8 == 7 && (dir == -7 || dir == 9) { continue; }
                            if index % 8 == 0 && (dir == -9 || dir == 7) { continue; }
                            if new_index < 0 || new_index > 63 { continue; }
                            if let Some(new_piece) = board.board[new_index as usize] {
                                if new_piece.color == piece.color { continue; }
                            } 
                            possible_moves.push(ChessMove { from: index, to: new_index as usize });
                        }
                        for dir in ROOK_DIR {
                            let new_index = index as i32 + dir;
                            if index / 8 == 0 && dir == -8 { continue; }
                            if index % 8 == 7 && dir == 1 { continue; }
                            if index % 8 == 0 && dir == -1 { continue; }
                            if index / 8 == 7 && dir == 8 { continue; }
                            if let Some(new_piece) = board.board[new_index as usize] {
                                if new_piece.color == piece.color { continue; }
                            } 
                            possible_moves.push(ChessMove { from: index, to: new_index as usize });
                        }
                    },
                    Queen => continue,
                    Rook(_) => continue,
                    Bishop => continue,
                    Knight => continue,
                    Pawn(_) => continue,
                }
            }
        }


        possible_moves
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "from: {} to: {}\n", self.from, self.to)
    }
}