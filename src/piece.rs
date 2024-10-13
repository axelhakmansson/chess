
#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    King(bool),
    Queen,
    Rook(bool),
    Bishop,
    Knight,
    Pawn(bool)
}
 
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub p_type: PieceType,
    pub color: Color
}

impl Piece {
    pub fn new(p_type: PieceType, color: Color) -> Piece{
        Piece {
            p_type,
            color
        }
    }

    pub fn black_pawn() -> Piece {
        Piece::new(PieceType::Pawn(false), Color::Black)
    }
    pub fn black_rook() -> Piece {
        Piece::new(PieceType::Rook(false), Color::Black)
    }
    pub fn black_knight() -> Piece {
        Piece::new(PieceType::Knight, Color::Black)
    }
    pub fn black_bishop() -> Piece {
        Piece::new(PieceType::Bishop, Color::Black)
    }
    pub fn black_queen() -> Piece {
        Piece::new(PieceType::Queen, Color::Black)
    }
    pub fn black_king() -> Piece {
        Piece::new(PieceType::King(false), Color::Black)
    }

    pub fn white_pawn() -> Piece {
        Piece::new(PieceType::Pawn(false), Color::White)
    }
    pub fn white_rook() -> Piece {
        Piece::new(PieceType::Rook(false), Color::White)
    }
    pub fn white_knight() -> Piece {
        Piece::new(PieceType::Knight, Color::White)
    }
    pub fn white_bishop() -> Piece {
        Piece::new(PieceType::Bishop, Color::White)
    }
    pub fn white_queen() -> Piece {
        Piece::new(PieceType::Queen, Color::White)
    }
    pub fn white_king() -> Piece {
        Piece::new(PieceType::King(false), Color::White)
    }
}
