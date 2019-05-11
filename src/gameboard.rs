/// Size of game board.
const SIZE : usize = 9;

/// Stores game board information
pub struct Gameboard{
    /// Stores the content of the cells
    /// `0` is an empty cell.
    cells : [[u8;SIZE];SIZE],
    /// Flags of readonly
    readonly : [[bool;SIZE]; SIZE],
}

impl Gameboard{
    /// Creates a new game board
    pub fn new() -> Gameboard{
        Gameboard{
            cells : [[0;SIZE];SIZE],
            readonly : [[false; SIZE];SIZE],
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
        if self.readonly[ind[1]][ind[0]] == false{
            self.cells[ind[1]][ind[0]] = val;
        }
    }

    /// Check whether valid or invalid
    pub fn invalid(&self)->bool{
        let invalid_pos = self.search_invalid_position();

        (0..SIZE*SIZE).map(|i| invalid_pos[i/SIZE][i%SIZE]).fold(false, |s,x| s | x)
    }

    /// Solve and fill answer.
    pub fn solve(&mut self){
        let answers = self.make_answer_list(1);

        if answers.len() > 0{
            self.cells = answers[0];
        }
    }
}

impl Gameboard{
    /// Search invalid area
    fn search_invalid_position(&self) -> [[bool;SIZE]; SIZE]{
        let mut invalid1 = self.search_invalid_position_section();
        let invalid2 = self.search_invalid_position_row();
        let invalid3 = self.search_invalid_position_col();

        for (i,j) in (0..SIZE*SIZE).map(|n| (n/SIZE,n%SIZE)){
            invalid1[i][j] |= invalid2[i][j] || invalid3[i][j];
        }
        
        invalid1
    }

    /// Search invalid position in specific area
    fn search_invalid_position_in(&self,inds : [(usize,usize);SIZE])->Vec<(usize,usize)>{
        let mut v = vec![];

        let counter = { 
            let mut temp = [0;SIZE+1];
            for (i,j) in inds.iter(){
                temp[ self.cells[*i][*j] as usize]+=1;
            }
            temp[0] = 0;

            temp
        };

        for (i,j) in inds.iter(){
            if counter[ self.cells[*i][*j] as usize] > 1{
                v.push((*i,*j));
            }
        }

        v
    }

    /// Search invalid area(section)
    fn search_invalid_position_section(&self)->[[bool;SIZE];SIZE]{
        let mut invalid = [[false;SIZE];SIZE];

        for (sec_i,sec_j) in (0..SIZE).map(|i| (i/3,i%3)){
            let area_index : [(usize,usize);SIZE] = {
                let mut temp = [(0,0); SIZE];

                for n in 0..SIZE{
                    temp[n] = (3*sec_i + (n/3), 3*sec_j + (n%3) );
                } 

                temp
            };
            for (i,j) in self.search_invalid_position_in(area_index){
                invalid[i][j] = true;
            }
        }

        invalid
    }

    /// Search invalid area(row)
    fn search_invalid_position_row(&self) -> [[bool;SIZE]; SIZE]{
        let mut invalid = [[false;SIZE];SIZE];

        for i in 0..SIZE{
            let area_index = {
                let mut temp = [(0,0);SIZE];
                for j in 0..SIZE{
                    temp[j] = (i,j);
                }
                temp
            };
            for (i,j) in self.search_invalid_position_in(area_index){
                invalid[i][j] = true;
            }
        }

        invalid
    }

    /// Search invalid area(col)
    fn search_invalid_position_col(&self) -> [[bool;SIZE]; SIZE]{
        let mut invalid = [[false;SIZE];SIZE];

        for j in 0..SIZE{
            let area_index = {
                let mut temp = [(0,0);SIZE];
                for i in 0..SIZE{
                    temp[i] = (i,j);
                }
                temp
            };

            for (i,j) in self.search_invalid_position_in(area_index){
                invalid[i][j] = true;
            }
        }

        invalid
    }

    /// Search all answers of sudoku(up to n_aswer answers).
    fn make_answer_list(&mut self,n_answer: usize) -> Vec<[[u8;SIZE]; SIZE]>{
        let mut v = vec![];
        self.solve_core(n_answer,0,&mut v);

        v
    }

    /// Core part of Sudoku solver
    fn solve_core(&mut self,n_answer: usize, position : usize, answers : &mut Vec<[[u8;SIZE];SIZE]> ){
        let (i,j) = (position/SIZE,position%SIZE);

        if self.invalid(){
            return;
        }

        if position == SIZE * SIZE{
            answers.push(self.cells.clone());
        }else if self.cells[i][j] != 0{
            self.solve_core(n_answer,position+1,answers);
        }else{
            for d in 1..SIZE+1{
                if n_answer <= answers.len(){
                    break;
                }

                self.cells[i][j] = d as u8;
                self.solve_core(n_answer,position+1,answers);
            }
            self.cells[i][j] = 0;
        }
    }
}

#[test]
fn test_invalid_col(){
    let mut gb = Gameboard::new();
    let mut invalid = [[false;SIZE];SIZE];

    gb.set([0,0],1);
    gb.set([0,1],1);
    invalid[0][0] = true;
    invalid[1][0] = true;
    assert_eq!(gb.search_invalid_position_col(),invalid);
}

#[test]
fn test_invalid_row(){
    let mut gb = Gameboard::new();
    let mut invalid = [[false;SIZE];SIZE];

    gb.set([0,0],1);
    gb.set([1,0],1);
    invalid[0][0] = true;
    invalid[0][1] = true;
    assert_eq!(gb.search_invalid_position_row(),invalid);
}

#[test]
fn test_invalid_section(){
    let mut gb = Gameboard::new();
    let mut invalid = [[false;SIZE];SIZE];

    gb.set([0,0],1);
    gb.set([1,0],1);
    invalid[0][0] = true;
    invalid[0][1] = true;
    assert_eq!(gb.search_invalid_position_section(),invalid);
}

#[test]
fn test_invalid_in(){
    let mut gb = Gameboard::new();
    
    gb.set([0,0],1);

    let area_index = [(0,0);SIZE];
    assert_eq!(gb.search_invalid_position_in(area_index),vec![(0,0);SIZE]);
}

