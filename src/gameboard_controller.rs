//! Gameboard controller.

use piston::input::GenericEvent;
use crate::gameboard::{Gameboard,SIZE};

/// Handles events for Sudoku game.
pub struct GameboardController{
    /// Stores the gameboard state.
    pub gameboard : Gameboard,
    /// Selected cell.
    pub selected_cell: Option<(usize,usize)>,
    /// Stores last mouse cursor position
    cursor_pos : [f64;2],
}

impl GameboardController{
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController{
        GameboardController{
            gameboard : gameboard,
            selected_cell : None,
            cursor_pos : [0.0,0.0],
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self,board_pos: [f64;2], board_size:f64, e:&E){
        use piston::input::{Button,MouseButton,Key};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args(){
            // find coordinates relative to upper left corner
            let x = self.cursor_pos[0] - board_pos[0];
            let y = self.cursor_pos[1] - board_pos[1];

            // Check hat coodinates are inside board boundaries.
            if x >= 0.0 && x < board_size && y >= 0.0 && y < board_size{
                // Compute the cell position
                let cell_x = (x / board_size * 9.0) as usize;
                let cell_y = (y / board_size * 9.0) as usize;
                self.selected_cell = Some((cell_y,cell_x));
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args(){
            if let Some(ind) = self.selected_cell{
                // Set cell value
                match key{
                    Key::D0 | Key::NumPad0 => self.gameboard.set(ind,0),
                    Key::D1 | Key::NumPad1 => self.gameboard.set(ind,1),
                    Key::D2 | Key::NumPad2 => self.gameboard.set(ind,2),
                    Key::D3 | Key::NumPad3 => self.gameboard.set(ind,3),
                    Key::D4 | Key::NumPad4 => self.gameboard.set(ind,4),
                    Key::D5 | Key::NumPad5 => self.gameboard.set(ind,5),
                    Key::D6 | Key::NumPad6 => self.gameboard.set(ind,6),
                    Key::D7 | Key::NumPad7 => self.gameboard.set(ind,7),
                    Key::D8 | Key::NumPad8 => self.gameboard.set(ind,8),
                    Key::D9 | Key::NumPad9 => self.gameboard.set(ind,9),
                    Key::Backspace => self.gameboard.set(ind,0),
                    Key::Delete => self.gameboard.set(ind,0),
                    Key::S => self.gameboard.solve(),
                    Key::G => self.gameboard.generate(),
                    Key::Up => self.arrow_key(-1, 0), 
                    Key::Right=> self.arrow_key(0, 1), 
                    Key::Left => self.arrow_key(0, -1), 
                    Key::Down => self.arrow_key(1, 0), 
                    _ => {},
                }
            }
        }
    }

    /// control selected_cell by arrow-key
    fn arrow_key(&mut self,dx:i32,dy:i32){
        if let Some(ind) = self.selected_cell{
            let (i,j) = ind;
            let x = i as i32 + dx + SIZE as i32;
            let y = j as i32 + dy + SIZE as i32;
            let x = (x as usize) % SIZE;
            let y = (y as usize) % SIZE;
            self.selected_cell = Some((x,y));
        }
    }
}