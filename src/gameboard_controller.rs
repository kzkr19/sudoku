//! Gameboard controller.

use piston::input::GenericEvent;
use crate::Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController{
    /// Stores the gameboard state.
    pub gameboard : Gameboard,
    /// Selected cell.
    pub selected_cell: Option<[usize;2]>,
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
        use piston::input::{Button,MouseButton};

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
                self.selected_cell = Some([cell_x,cell_y]);
            }
        }
    }
}