extern crate piston;
extern crate glutin_window;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use piston::event_loop::{Events,EventSettings,EventLoop};

fn main() {
    let settings = WindowSettings::new("Sudoku",[512;2])
        .exit_on_esc(true);
    let mut window : GlutinWindow = settings.build()
        .expect("Cound not create window");
        
    let mut events = Events::new(EventSettings::new().lazy(true));
}