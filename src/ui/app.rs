use eframe::egui;

use crate::engine::{BacktrackingSolver, Grid, SolveOutcome, SudokuSolver};
use super::{draw::draw_sudoku_grid, input::handle_digit_keys, presets::preset_easy};

pub struct SudokuApp {
    grid: [u8; 81],
    selected: Option<usize>,

    solver: BacktrackingSolver,
    message: String,
    last_solve_ms: Option<f64>,
    cycle_limit: usize,

    givens: [bool; 81],
}

impl SudokuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(0.5);

        Self {
            grid: [0; 81],
            selected: None,
            solver: BacktrackingSolver::new(),
            message: String::new(),
            last_solve_ms: None,
            cycle_limit: 1_000_000_000,
            givens: [false; 81],
        }
    }

    fn load_preset(&mut self, p: [u8; 81]) {
        self.grid = p;
        self.selected = None;
        self.givens = self.grid.map(|v| v != 0);
        self.message.clear();
        self.last_solve_ms = None;
    }

    fn clear(&mut self) {
        self.grid = [0; 81];
        self.givens = [false; 81];
        self.selected = None;
        self.message.clear();
        self.last_solve_ms = None;
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        handle_digit_keys(ctx, &mut self.grid, self.selected, &self.givens);

        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();

            let margin_x = (available.x * 0.05) as i8;
            let margin_y = (available.y * 0.05) as i8;

            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(margin_x, margin_y))
                .show(ui, |ui| {
                    let available = ui.available_size();
                    let board_area = egui::vec2(available.x, available.y * 0.5);

                    ui.allocate_ui_with_layout(
                        board_area,
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            draw_sudoku_grid(ui, &self.grid, &mut self.selected);
                        },
                    );
                });

            ui.horizontal(|ui| {
                if ui.button("Easy").clicked() {
                    self.load_preset(preset_easy());
                }
                if ui.button("Clear").clicked() {
                    self.clear();
                }

                if ui.button("Solve").clicked() {
                    // convert [u8;81] to engine::Grid
                    let mut g = match Grid::from_array(self.grid) {
                        Ok(g) => g,
                        Err(_) => {
                            self.message = "Grid has invalid values (>9)".into();
                            return;
                        }
                    };

                    let start = std::time::Instant::now();
                    let outcome = self.solver.solve(&mut g, self.cycle_limit);
                    self.last_solve_ms = Some(start.elapsed().as_secs_f64() * 1000.0);

                    match outcome {
                        SolveOutcome::Solved => {
                            self.grid = *g.as_array();
                            self.message = "Solved.".into();
                        }
                        SolveOutcome::InvalidGiven => {
                            self.message = "Invalid puzzle (conflict in givens)".into();
                        }
                        SolveOutcome::Unsolved => {
                            self.message = "No solution.".into();
                        }
                        SolveOutcome::CycleLimit => {
                            self.message = "No solution / limit reached.".into();
                        }
                    }
                }
            });

            if let Some(ms) = self.last_solve_ms {
                ui.label(format!("Last solve: {:.2} ms", ms));
            }
            ui.label(&self.message);
        });
    }
}
