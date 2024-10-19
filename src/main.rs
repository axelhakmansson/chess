use std::cmp;

use glutin_window::GlutinWindow;

use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, MouseButton};
use piston::{EventLoop, RenderEvent, WindowSettings};
use piston::{MouseCursorEvent, PressEvent};

use crate::game::Game;

pub mod piece;
pub mod game;
pub mod board;
pub mod chess_move;
pub mod render;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;
const CELL_SIZE: f64 = SCREEN_HEIGHT as f64 / 8.0;

fn main() {
    let mut game = Game::new_standard();
    println!("{}", game);

    let opengl = OpenGL::V3_2;
    // The piston window
    let mut window: GlutinWindow =
        WindowSettings::new("Chess", [SCREEN_WIDTH, SCREEN_HEIGHT])
            .graphics_api(opengl)
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut event = Events::new(EventSettings::new().lazy(false));
    let mut gl = GlGraphics::new(opengl);
    let mut mouse_position: [f64; 2] = [0.0, 0.0];

    // import the font
    let font = "fonts/DejaVuSansCondensed-Bold.ttf";
    // Create a piston specific 'Glyphs' from the font
    let mut glyphs =
        GlyphCache::new(font, (), TextureSettings::new()).expect("Could not load font");

    let mut selected: Vec<usize> = Vec::new();
    
    while let Some(e) = event.next(&mut window) {
        
        mouse_position = if let Some(position) = e.mouse_cursor_args() { position } else { mouse_position };

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            selected = vec![];
            let x: usize = cmp::min(
                (mouse_position[0] / CELL_SIZE) as usize,
                (SCREEN_WIDTH - 1) as usize,
            );
            let y: usize = cmp::min(
                (mouse_position[1] / CELL_SIZE) as usize,
                (SCREEN_HEIGHT - 1) as usize,
            );

            let possible = game.get_possible_moves();

            for i in 0..possible.len() {
                if possible[i].from == x + 8 * y {
                    selected.push(possible[i].to);
                }
            }
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                // clear the screen
                use graphics::clear;
                clear([1.0; 4], g);
                // draw board
                render::draw_board(&c, g, &game.board, &selected, &mut glyphs);
            });
        }
    }
}
