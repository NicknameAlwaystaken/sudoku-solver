use super::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolveOutcome {
    Solved,
    Unsolved,        // no solution exists
    CycleLimit,      // gave up due to cycle limit
    InvalidGiven,    // initial grid already invalid
}

pub trait SudokuSolver {
    fn name(&self) -> &'static str { "solver" }      // optional, handy for UI
    fn cycles_done(&self) -> usize;

    fn solve(&mut self, grid: &mut Grid, cycle_limit: usize) -> SolveOutcome;
}
