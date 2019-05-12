use crate::gameboard::{SIZE,Gameboard};
use rand::prelude::*;

/// Sudoku generator
pub struct Generator{
}

impl Generator{
    /// Create new Generator
    pub fn new()->Generator{
        Generator{
        }
    }

    /// Create new Sudoku quiz
    pub fn generate(&mut self,gb:&mut Gameboard){
       gb.reset();
       gb.solve();

       self.make_hole(gb,SIZE*SIZE*9/16);
       self.set_readonly(gb);
    }

    /// Delete several cells
    fn make_hole(&mut self,gb:&mut Gameboard,n_max_hole: usize){
        let mut cnt = 0;
        let mut v = (0..SIZE*SIZE)
            .map(|n| (n%SIZE,n/SIZE))
            .collect::<Vec<(usize,usize)>>();

        v.shuffle(&mut rand::thread_rng());

        for pos in v{
            let original = gb.get(pos);
            // remove digit
            gb.set(pos,0);

            if gb.unique() == false{
                // undo
                gb.set(pos,original);
            }else{
                // count removed digit
                cnt+=1;
            }

            if cnt == n_max_hole{
                break;
            }
        }
    }

    /// set readonly value to gameboard
    fn set_readonly(&self,gb:&mut Gameboard){
        for i in 0..SIZE{
            for j in 0..SIZE{
                let f = gb.get((i,j)) != 0;
                gb.set_readonly((i,j),f);
            }
        }
    }
}