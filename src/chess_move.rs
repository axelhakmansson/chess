use crate::board::Board;
use crate::piece::Color;
use crate::piece::PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
use std::fmt::{self, Display};

const BISHOP_DIR: [i32; 4] = [-9, -7, 7, 9];
const ROOK_DIR: [i32; 4] = [-8, -1, 1, 8];
const KNIGHT_DIR: [i32; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];
const WHITE_PAWN_DIR: [i32; 4] = [-8, -16, -7, -9];
const BLACK_PAWN_DIR: [i32; 4] = [8, 16, 7, 9];

#[derive(Debug, Clone)]
pub struct ChessMove {
    pub from: usize,
    pub to: usize
}

impl ChessMove {
    fn new(from: usize, to: usize) -> ChessMove {
        ChessMove {
            from, 
            to
        }
    }

    fn get_legal_moves(board: &Board, col: Color) -> Vec<ChessMove> {
        let mut possible_moves = ChessMove::get_possible_moves(board, col);

        let mut legal_moves = Vec::new();
        legal_moves
    }

    pub fn get_possible_moves(board: &Board, col: Color) -> Vec<ChessMove>{
        let mut possible_moves = Vec::new();

        for index in 0..board.board.len() {
            if let Some(piece) = board.board[index] {
                if piece.color == col {
                    match piece.p_type {
                        King(_) => possible_moves.append(&mut Self::king_moves(board, index, col)),
                        Queen => possible_moves.append(&mut Self::queen_moves(board, index, col)),
                        Rook(_) => possible_moves.append(&mut Self::rook_moves(board, index, col)),
                        Bishop => possible_moves.append(&mut Self::bishop_moves(board, index, col)),
                        Knight => possible_moves.append(&mut Self::knight_moves(board, index, col)),
                        Pawn(moved) => {
                            if col == Color::White {
                                possible_moves.append(&mut Self::white_pawn_moves(board, index, moved));
                            } else if col == Color::Black {
                                possible_moves.append(&mut Self::black_pawn_moves(board, index, moved));
                            }
                        },
                    }
                }
            }
        }
        possible_moves      
    }

    fn king_moves(board: &Board, index: usize, col: Color) -> Vec<ChessMove> {
        let mut possible_moves = Vec::new();
        for dir in BISHOP_DIR {
            let new_index = index as i32 + dir;
            if index % 8 == 7 && (dir == -7 || dir == 9) { continue; }
            if index % 8 == 0 && (dir == -9 || dir == 7) { continue; }
            if new_index < 0 || new_index > 63 { continue; }
            if let Some(new_piece) = board.board[new_index as usize] {
                if new_piece.color == col { continue; }
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
                if new_piece.color == col { continue; }
            } 
            possible_moves.push(ChessMove { from: index, to: new_index as usize });
        }
        possible_moves  
    }

    fn queen_moves(board: &Board, index: usize, col: Color) -> Vec<ChessMove> {
        let mut possible_moves = Vec::new();
        possible_moves.append(&mut Self::rook_moves(board, index, col));
        possible_moves.append(&mut Self::bishop_moves(board, index, col));
        possible_moves 
    }

    fn rook_moves(board: &Board, index: usize, col: Color) -> Vec<ChessMove> {
        let mut possible_moves = Vec::new();

        for dir in ROOK_DIR {
            for i in 1..8 {
                let new_index = index as i32 + (dir * i);
                if new_index < 64 && new_index >= 0 {
                    if (new_index % 8 == 0 && index % 8 == 7) || (new_index % 8 == 7 && index % 8 == 0) {
                        break;
                    }
                    if let Some(piece) = board.board[new_index as usize]{
                        if piece.color == col { break; }
                        else {
                            possible_moves.push(ChessMove { from: index, to: new_index as usize });
                            break;
                        }
                    }
                    possible_moves.push(ChessMove { from: index, to: new_index as usize });
                }
            }
        }
        possible_moves 
    }

    fn bishop_moves(board: &Board, index: usize, col: Color) -> Vec<ChessMove> {
        let mut possible_moves = Vec::new();

        for dir in BISHOP_DIR {
            for i in 1..8 {
                let new_index = index as i32 + (dir * i);
                if new_index < 64 && new_index >= 0 {
                    if index % 8 == 0 && (dir == -9 || dir == 7) { break; }
                    if index % 8 == 7 && (dir == 9 || dir == -7) { break; }
                    if new_index % 8 == 0 || new_index % 8 == 7 {
                        possible_moves.push(ChessMove { from: index, to: new_index as usize });
                        break;
                    }
                    if let Some(piece) = board.board[new_index as usize]{
                        if piece.color == col { break; }
                        else {
                            possible_moves.push(ChessMove { from: index, to: new_index as usize });
                            break;
                        }
                    }
                    possible_moves.push(ChessMove { from: index, to: new_index as usize });
                }
            }
        }
        possible_moves 
    }

    fn knight_moves(board: &Board, index: usize, col: Color) -> Vec<ChessMove> {
        let mut possible_moves = Vec::new();
        
        for dir in KNIGHT_DIR {
            let new_index = index as i32 + dir;
            if new_index < 64 && new_index >= 0 && (index % 8).abs_diff((new_index % 8) as usize) < 3 {
                if let Some(piece) = board.board[new_index as usize] {
                    if piece.color == col { continue; }
                }
                possible_moves.push(ChessMove { from:index, to: new_index as usize });
            }
        }
        possible_moves 
    }

    fn black_pawn_moves(board: &Board, index: usize, moved: bool) -> Vec<ChessMove> {
        let mut possible_moves = Vec::new();
        let mut in_front = false;
        for dir in BLACK_PAWN_DIR {
            if moved && dir == 16 { continue; }
            let new_index = index as i32 + dir;
            if new_index < 64 {
                if let Some(piece) = board.board[new_index as usize] {
                    if piece.color != Color::Black && ((index % 8 != 0 && dir == 7) || (index % 8 != 7 && dir == 9)) {
                        possible_moves.push(ChessMove { from: index, to: new_index as usize });
                    } else { 
                        in_front = true; 
                        continue; 
                    }
                } else if dir != 7 && dir != 9 && !in_front {
                    possible_moves.push(ChessMove { from: index, to: new_index as usize });
                }                
            }
        }
        possible_moves 
    }

    fn white_pawn_moves(board: &Board, index: usize, moved: bool) -> Vec<ChessMove> {
        let mut possible_moves = Vec::new();
        let mut in_front = false;
        for dir in WHITE_PAWN_DIR {
            if moved && dir == -16 { continue; }
            let new_index = index as i32 + dir;
            if new_index >= 0 {
                if let Some(piece) = board.board[new_index as usize] {
                    if piece.color != Color::White && ((index % 8 != 7 && dir == -7) || (index % 8 != 0 && dir == -9)) {
                        possible_moves.push(ChessMove { from: index, to: new_index as usize });
                    } else { 
                        in_front = true; 
                        continue; 
                    }
                } else if dir != -7 && dir != -9 && !in_front {
                    possible_moves.push(ChessMove { from: index, to: new_index as usize });
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