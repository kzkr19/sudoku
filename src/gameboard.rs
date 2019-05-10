/// Size of game board.
const SIZE : usize = 9;

/// Stores game board information
pub struct Gameboard{
    /// Stores the content of the cells
    /// `0` is an empty cell.
    pub cells : [[u8;SIZE];SIZE],
}

impl Gameboard{
    /// Creates a new game board
    pub fn new() -> Gameboard{
        Gameboard{
            cells : [[0;SIZE];SIZE],
        }
    }

    /// Gets the character at cell location
    pub fn char(&self,ind:[usize;2]) -> Option<char>{
        match self.cells[ind[1]][ind[0]]{
            n if 1<= n && n <= 9 => 
                Some(('0' as u8 + n) as char),
            _ => 
                None,
        }
    }

    /// Set cell value.
    pub fn set(&mut self,ind:[usize;2],val : u8){
        self.cells[ind[1]][ind[0]] = val;
    }
}