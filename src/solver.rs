//! A program of Sudoku solver

use rand::prelude::*;

/// Sudoku solver
pub struct Solver{
    /// random number generator
    rng : ThreadRng,
}

use crate::gameboard::{SIZE,Gameboard};

impl Solver{
    /// Create new Solver
    pub fn new()->Solver{
        Solver{
            rng : rand::thread_rng()
        }
    }

    /// Search all answers of sudoku(up to n_aswer answers).
    pub fn make_answer_list(&mut self, gb: &mut Gameboard,n_answer: usize) -> Vec<[[u8;SIZE]; SIZE]>{
        let mut v = vec![];
        self.solve_core(gb,n_answer,0,&mut v);

        v
    }

    /// Core part of Sudoku solver
    fn solve_core(&mut self,gb: &mut Gameboard,n_answer: usize, position : usize, answers : &mut Vec<[[u8;SIZE];SIZE]> ){
        let (i,j) = (position/SIZE,position%SIZE);

        if gb.invalid(){
            return;
        }

        if position == SIZE * SIZE{
            answers.push(gb.copy_cells());
        }else if gb.get((i,j))!= 0{
            self.solve_core(gb, n_answer,position+1,answers);
        }else{
            let mut vec = (1..SIZE+1).collect::<Vec<usize>>();
            vec.shuffle(&mut self.rng);

            for d in vec{
                if n_answer <= answers.len(){
                    break;
                }

                gb.set((i,j),d as u8);
                self.solve_core(gb,n_answer,position+1,answers);
            }
            gb.set((i,j),0);
        }
    }
}