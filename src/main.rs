#![deny(missing_docs)]

//! A Sudoku game
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::{Events,EventSettings,EventLoop};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL,GlGraphics,Filter,GlyphCache,TextureSettings};

pub use crate::gameboard::Gameboard;
pub use crate::gameboard_controller::GameboardController;
pub use crate::gameboard_view::{GameboardView,GameboardViewSettings};
pub use crate::solver::Solver;
pub use crate::generator::Generator;

mod gameboard;
mod gameboard_controller;
mod gameboard_view;
mod solver;
mod generator;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku",[512;2])
        .exit_on_esc(true)
        .opengl(opengl);
    let mut window : GlutinWindow = settings.build()
        .expect("Cound not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let mut gameboard = Gameboard::new();
    gameboard.generate();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Cound not load font.");

    while let Some(e) = events.next(&mut window){
        gameboard_controller.event(gameboard_view.settings.position,gameboard_view.settings.size,&e);

        if let Some(args) = e.render_args(){
            gl.draw( args.viewport(), |c,g|{
                use graphics::{clear};

                clear([1.0;4],g);
                gameboard_view.draw(&gameboard_controller,glyphs,&c,g);
            });
        }
    }
}