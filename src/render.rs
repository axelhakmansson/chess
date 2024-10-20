use graphics::{Context, Graphics};
use graphics::{Rectangle, Transformed};
use opengl_graphics::*;

use crate::board::Board;
use crate::piece::Color;
use crate::piece::PieceType::{King,Knight,Queen,Bishop,Rook,Pawn};
use crate::SCREEN_HEIGHT;

const WHITE: [f32; 4] = [252.0 / 255.0, 255.0 / 255.0, 217.0 / 255.0, 1.0];
const BLACK: [f32; 4] = [14.0 / 255.0, 135.0 / 255.0, 16.0 / 255.0, 1.0];
const WHITE_SELECTED: [f32; 4] = [1.0, 0.5, 0.43, 0.5];
const BLACK_SELECTED: [f32; 4] = [1.0, 135.0 / 255.0 / 2.0, 16.0 / 255.0 / 2.0, 0.5];
const CELL_SIZE: f64 = SCREEN_HEIGHT as f64 / 8.0;

pub fn draw_board<G: Graphics<Texture = Texture>>(
    c: &Context,
    g: &mut G,
    board: &Board,
    to: &Vec<usize>,
    glyphs: &mut GlyphCache
) {
    let mut count = 0;
    for i in 0..64 {
        
        let mut color = if count % 2 == 0 { WHITE } else { BLACK };

        count += if i % 8 == 7 { 1 } else { 0 };

        let x = (i % 8) as f64 * CELL_SIZE;
        let y = (i / 8) as f64 * CELL_SIZE;

        if to.contains(&i) {
            color = if color == WHITE {WHITE_SELECTED} else {BLACK_SELECTED};
            Rectangle::new(color).draw(
                [x, y, CELL_SIZE, CELL_SIZE],
                &c.draw_state,
                c.transform,
                g,
            );
        }

        Rectangle::new(color).draw(
            [x, y, CELL_SIZE, CELL_SIZE],
            &c.draw_state,
            c.transform,
            g,
        );

        if let Some(piece) = &board.board[i] {
            let name;
            match piece.color {
                Color::White => {
                    match piece.p_type {
                        Pawn(_) => name = '\u{2659}', // White Pawn
                        Rook(_) => name = '\u{2656}', // White Rook
                        Knight => name = '\u{2658}', // White Knight
                        Bishop => name = '\u{2657}', // White Bishop
                        Queen => name = '\u{2655}', // White Queen
                        King(_) => name = '\u{2654}', // White King
                    }
                }
                Color::Black => {
                    match piece.p_type {
                        Pawn(_) => name = '\u{265F}', // White Pawn
                        Rook(_) => name = '\u{265C}', // White Rook
                        Knight => name = '\u{265E}', // White Knight
                        Bishop => name = '\u{265D}', // White Bishop
                        Queen => name = '\u{265B}', // White Queen
                        King(_) => name = '\u{265A}', // White King
                    }
                }
            }
            let piece_transform = c.transform.trans(
                x + CELL_SIZE / 2.0 - 24.0, // 24 is about half the width of the character
                y + CELL_SIZE / 2.0 + 24.0,
            );
            // A transform to move the placement of the text
            graphics::text::Text::new(50)
                .draw(&name.to_string(), glyphs, &c.draw_state, piece_transform, g)
                .unwrap();

            // For some reason this is needed for the text to render correctly
            // draws a space in another color
            graphics::text::Text::new_color([1.0; 4], 32)
                .draw(" ", glyphs, &c.draw_state, c.transform, g)
                .unwrap();
        }
        count += 1;
    }
}