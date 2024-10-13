use crate::piece::Piece;

#[derive(Debug)]
pub struct Board {
    pub board: [Option<Piece>; 64]
}

impl Board {
    pub fn standard() -> Board {
        let mut board = [None; 64];
        board[0] = Some(Piece::black_rook());
        board[1] = Some(Piece::black_knight());
        board[2] = Some(Piece::black_bishop());
        board[3] = Some(Piece::black_queen());
        board[4] = Some(Piece::black_king());
        board[5] = Some(Piece::black_bishop());
        board[6] = Some(Piece::black_knight());
        board[7] = Some(Piece::black_rook());

        for i in 8..=15 {
            board[i] = Some(Piece::black_pawn());
        }


        for i in 48..=55 {
            board[i] = Some(Piece::white_pawn());
        }

        board[56] = Some(Piece::white_rook());
        board[57] = Some(Piece::white_knight());
        board[58] = Some(Piece::white_bishop());
        board[59] = Some(Piece::white_queen());
        board[60] = Some(Piece::white_king());
        board[61] = Some(Piece::white_bishop());
        board[62] = Some(Piece::white_knight());
        board[63] = Some(Piece::white_rook());
        Board {
            board
        }
    }
}