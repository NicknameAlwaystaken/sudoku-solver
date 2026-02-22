use super::{grid::is_placement_valid, Grid};
use super::solver::{SolveOutcome, SudokuSolver};

#[derive(Clone, Copy)]
struct Step {
    empty_i: usize,
    next: u8,
}

pub struct BacktrackingSolver {
    cycles_done: usize,
}

impl BacktrackingSolver {
    pub fn new() -> Self {
        Self { cycles_done: 0 }
    }
}

impl SudokuSolver for BacktrackingSolver {
    fn cycles_done(&self) -> usize {
        self.cycles_done
    }

    fn solve(&mut self, grid: &mut Grid, cycle_limit: usize) -> SolveOutcome {
        self.cycles_done = 0;

        if !grid.is_valid_given() {
            return SolveOutcome::InvalidGiven;
        }

        let cells = grid.as_array_mut();

        let mut empty_locations: Vec<usize> = Vec::new();
        for idx in 0..81 {
            if cells[idx] == 0 {
                empty_locations.push(idx);
            }
        }

        if empty_locations.is_empty() {
            return SolveOutcome::Solved;
        }

        let mut steps: Vec<Step> = Vec::new();
        let mut empty_i: usize = 0;
        let mut next_try: u8 = 1;

        while self.cycles_done < cycle_limit {
            self.cycles_done += 1;

            let cell_idx = empty_locations[empty_i];
            cells[cell_idx] = 0;

            let mut placed = false;

            while next_try <= 9 {
                if is_placement_valid(cells, cell_idx, next_try) {
                    cells[cell_idx] = next_try;

                    steps.push(Step { empty_i, next: next_try + 1 });
                    empty_i += 1;
                    next_try = 1;
                    placed = true;
                    break;
                }
                next_try += 1;
            }

            if placed {
                if empty_i == empty_locations.len() {
                    return SolveOutcome::Solved;
                }
                continue;
            }

            let Some(prev) = steps.pop() else {
                return SolveOutcome::Unsolved;
            };

            empty_i = prev.empty_i;
            next_try = prev.next;
        }

        SolveOutcome::CycleLimit
    }
}
