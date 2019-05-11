//! A program of Sudoku solver

/// Sudoku solver
pub struct Solver;

use crate::gameboard::{SIZE,Gameboard};

impl Solver{
    /// Search all answers of sudoku(up to n_aswer answers).
    pub fn make_answer_list(gb: &mut Gameboard,n_answer: usize) -> Vec<[[u8;SIZE]; SIZE]>{
        let mut v = vec![];
        Solver::solve_core(gb,n_answer,0,&mut v);

        v
    }

    /// Core part of Sudoku solver
    fn solve_core(gb: &mut Gameboard,n_answer: usize, position : usize, answers : &mut Vec<[[u8;SIZE];SIZE]> ){
        let (i,j) = (position/SIZE,position%SIZE);

        if gb.invalid(){
            return;
        }

        if position == SIZE * SIZE{
            answers.push(gb.copy_cells());
        }else if gb.get((i,j))!= 0{
            Solver::solve_core(gb, n_answer,position+1,answers);
        }else{
            for d in 1..SIZE+1{
                if n_answer <= answers.len(){
                    break;
                }

                gb.set((i,j),d as u8);
                Solver::solve_core(gb,n_answer,position+1,answers);
            }
            gb.set((i,j),0);
        }
    }
}