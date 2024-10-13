use crate::piece::Color;
use crate::piece::PieceType;
use crate::board::Board;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum GameState {
    OnGoing,
    Check,
    CheckMate,
    StaleMate,
    Promotion
}

#[derive(Debug)]
pub struct Game {
    turn: Color,
    sate: GameState,
    pub board: Board
}

impl Game {
    pub fn new_standard() -> Game {
        Game {
            turn: Color::White,
            sate: GameState::OnGoing,
            board: Board::standard()
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        res.push_str(&format!("{}", "\nState: "));
        match self.sate {
            GameState::OnGoing => res.push_str(&format!("{}", "OnGoing\n")),
            GameState::Check => res.push_str(&format!("{}", "Check\n")),
            GameState::CheckMate => res.push_str(&format!("{}", "Checkmate\n")),
            GameState::StaleMate => res.push_str(&format!("{}", "Stalemate\n")),
            GameState::Promotion => res.push_str(&format!("{}", "Promotion\n")),
        }

        match self.turn {
            Color::White => res.push_str(&format!("{}", "Turn: White\nBlack is up white is down (terminal is weird)\n\n")),
            Color::Black => res.push_str(&format!("{}", "Turn: Black\nBlack is up white is down (terminal is weird)\n\n")),
        }

        for i in 0..64 {
            if let Some(piece) = self.board.board[i]  {
                match piece.color {
                    Color::White => 
                        match piece.p_type {
                            PieceType::King(_) => res.push_str(&format!("{:2}{}", "", "♔")),
                            PieceType::Queen => res.push_str(&format!("{:2}{}", "", "♕")),
                            PieceType::Bishop => res.push_str(&format!("{:2}{}", "", "♗")),
                            PieceType::Knight => res.push_str(&format!("{:2}{}", "", "♘")),
                            PieceType::Rook(_) => res.push_str(&format!("{:2}{}", "", "♖")),
                            PieceType::Pawn(_) => res.push_str(&format!("{:2}{}", "", "♙")),
                        }
                    Color::Black => 
                        match piece.p_type {
                            PieceType::King(_) => res.push_str(&format!("{:2}{}", "", "♚")),
                            PieceType::Queen => res.push_str(&format!("{:2}{}", "", "♛")),
                            PieceType::Bishop => res.push_str(&format!("{:2}{}", "", "♝")),
                            PieceType::Knight => res.push_str(&format!("{:2}{}", "", "♞")),
                            PieceType::Rook(_) => res.push_str(&format!("{:2}{}", "", "♜")),
                            PieceType::Pawn(_) => res.push_str(&format!("{:2}{}", "", "♟")),
                        }
                    }
            } else {
                res.push_str(&format!("{:2}{}", "", "_"));
            }
            if ((i + 1) % 8) == 0{
                res.push_str(&format!("{}", "\n"));
            }
        }
        write!(f, "{}", res)
    }
}