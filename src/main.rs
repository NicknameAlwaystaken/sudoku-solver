use clap::Parser;
use eframe::egui;


struct Solver {
    cycles_done: usize,
}

impl Solver {
    fn new() -> Self {
        Self { cycles_done: 0 }
    }

    fn solve(&mut self, grid: &mut [u8; 81], cycle_limit: usize) -> bool {
        self.cycles_done = 0;

        let mut empty_locations: Vec<usize> = Vec::new();

        for idx in 0..81 {
            if grid[idx] == 0 {
                empty_locations.push(idx);
            }
        }

        if empty_locations.is_empty() {
            println!("Puzzle is already solved!");
            return true;
        }

        let mut steps: Vec<Step> = Vec::new();

        let mut empty_i: usize = 0;
        let mut next_try: u8 = 1;

        while self.cycles_done < cycle_limit {
            self.cycles_done += 1;

            let cell_idx = empty_locations[empty_i];
            grid[cell_idx] = 0;

            let mut placed = false;

            while next_try <= 9 {
                if check_pos_for_valid(grid, cell_idx, next_try) {
                    grid[cell_idx] = next_try;

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
                    return true;
                }
                continue;
            }

            let Some(prev) = steps.pop() else {
                return false;
            };

            empty_i = prev.empty_i;
            next_try = prev.next;
        }

        false
    }

    fn check_sudoku_valid(grid: &[u8; 81]) -> bool {
        for idx in 0..81 {
            let val = grid[idx];
            if val == 0 { continue; }
            if !check_pos_for_valid(grid, idx, val) {
                return false;
            }
        }
        true
    }
}

fn check_pos_for_valid(grid: &[u8; 81], idx: usize, val: u8) -> bool {
    debug_assert!(val >= 1 && val <= 9);

    let row = idx / 9;
    let col = idx % 9;

    for c in 0..9 {
        let i = get_idx(row, c);
        if i != idx && grid[i] == val {
            return false;
        }
    }

    for r in 0..9 {
        let i = get_idx(r, col);
        if i != idx && grid[i] == val {
            return false;
        }
    }

    let box_r = (row / 3) * 3;
    let box_c = (col / 3) * 3;

    for r in box_r..box_r + 3 {
        for c in box_c..box_c + 3 {
            let i = get_idx(r, c);
            if i != idx && grid[i] == val {
                return false;
            }
        }
    }

    true
}

struct SudokuApp {
    grid: [u8; 81],
    selected: Option<usize>,

    solver: Solver,
    message: String,
    last_solve_ms: Option<f64>,
    cycle_limit: usize,

    givens: [bool; 81], // optional: lock original cells
}

impl SudokuApp {
    fn new(cc: &eframe::CreationContext<'_>, args: Args) -> Self {
        cc.egui_ctx.set_pixels_per_point(0.5);

        Self {
            grid: [0; 81],
            selected: None,
            solver: Solver::new(),
            message: String::new(),
            last_solve_ms: None,
            cycle_limit: 1_000_000_000,
            givens: [false; 81],
        }
    }

    fn load_preset(&mut self, p: [u8;81]) {
        self.grid = p;
        self.selected = None;
        self.givens = self.grid.map(|v| v != 0);
        self.message.clear();
        self.last_solve_ms = None;
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        handle_digit_keys(ctx, &mut self.grid, self.selected);

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let available = ui.available_size();

                let margin_x = (available.x * 0.05) as i8;
                let margin_y = (available.y * 0.05) as i8;

                egui::Frame::none()
                    .inner_margin(egui::Margin::symmetric(margin_x, margin_y))
                    .show(ui, |ui| {
                        let available = ui.available_size();
                        let board_area = egui::vec2(available.x, available.y * 0.5);

                        ui.allocate_ui_with_layout(
                            board_area,
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                draw_sudoku_grid(ui, &mut self.grid, &mut self.selected);
                            },
                        );

                    });

                ui.horizontal(|ui| {
                    if ui.button("Easy").clicked() { self.load_preset(preset_easy()); }
                    if ui.button("Clear").clicked() {
                        self.grid = [0;81];
                        self.givens = [false;81];
                        self.selected = None;
                    }

                    if ui.button("Solve").clicked() {
                        if !Solver::check_sudoku_valid(&self.grid) {
                            self.message = "Invalid puzzle (conflict in givens)".into();
                        } else {
                            let start = std::time::Instant::now();
                            let solved = self.solver.solve(&mut self.grid, self.cycle_limit);
                            self.last_solve_ms = Some(start.elapsed().as_secs_f64() * 1000.0);
                            self.message = if solved { "Solved.".into() } else { "No solution / limit reached.".into() };
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

fn preset_easy() -> [u8;81] {
    [
        5,3,0, 0,7,0, 0,0,0,
        6,0,0, 1,9,5, 0,0,0,
        0,9,8, 0,0,0, 0,6,0,
        8,0,0, 0,6,0, 0,0,3,
        4,0,0, 8,0,3, 0,0,1,
        7,0,0, 0,2,0, 0,0,6,
        0,6,0, 0,0,0, 2,8,0,
        0,0,0, 4,1,9, 0,0,5,
        0,0,0, 0,8,0, 0,7,9,
    ]
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    verbose: bool,
}

struct Step {
    empty_i: usize,
    next: u8,
}

fn draw_sudoku_grid(
    ui: &mut egui::Ui,
    grid: &mut [u8; 81],
    selected: &mut Option<usize>,
) {
    let outer_gap = 12.0;
    let inner_gap = 4.0;

    let available = ui.available_size();
    let board_size = available.x.min(available.y);

    let total_gap = 6.0 * inner_gap + 2.0 * outer_gap;

    let cell = ((board_size - total_gap) / 9.0).floor().max(10.0);
    let cell_size = egui::vec2(cell, cell);

    egui::Grid::new("outer_grid")
        .spacing(egui::vec2(outer_gap, outer_gap)) // spacing between 3x3 boxes
        .show(ui, |ui| {
            for box_row in 0..3 {
                for box_col in 0..3 {
                    ui.vertical(|ui| {
                        egui::Grid::new(format!("inner_{}_{}", box_row, box_col))
                            .spacing(egui::vec2(inner_gap, inner_gap))
                            .show(ui, |ui| {
                                for r in 0..3 {
                                    for c in 0..3 {
                                        let global_row = box_row * 3 + r;
                                        let global_col = box_col * 3 + c;
                                        let idx = global_row * 9 + global_col;

                                        let label = match grid[idx] {
                                            0 => " ",
                                            1 => "1",
                                            2 => "2",
                                            3 => "3",
                                            4 => "4",
                                            5 => "5",
                                            6 => "6",
                                            7 => "7",
                                            8 => "8",
                                            9 => "9",
                                            _ => "?",
                                        };

                                        let is_selected = *selected == Some(idx);

                                        let resp = cell_button(ui, cell_size, &label, is_selected);

                                        if resp.clicked() {
                                            *selected = Some(idx);
                                            resp.request_focus();
                                        }
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                }
                ui.end_row();
            }
        });
}

fn cell_button(ui: &mut egui::Ui, size: egui::Vec2, label: &str, selected: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    // Hover/pressed visuals come from the style interaction
    let interact = ui.style().interact(&resp);
    let visuals = ui.visuals();

    let (fill, stroke) = if selected {
        (visuals.selection.bg_fill, visuals.selection.stroke)
    } else {
        (interact.bg_fill, interact.bg_stroke)
    };

    ui.painter().rect(
        rect,
        2.0,
        fill,
        stroke,
        egui::StrokeKind::Middle,
    );

    let font_size = (size.y * 0.75).clamp(10.0, 40.0);
    let font_id = egui::FontId::monospace(font_size);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        font_id,
        visuals.text_color(),
    );

    resp
}

fn handle_digit_keys(ctx: &egui::Context, grid: &mut [u8; 81], selected: Option<usize>) {
    let Some(idx) = selected else { return };

    ctx.input(|i| {
        if i.key_pressed(egui::Key::Backspace) {
            grid[idx] = 0;
        }

        let d = if i.key_pressed(egui::Key::Num1) { Some(1) }
        else if i.key_pressed(egui::Key::Num2) { Some(2) }
        else if i.key_pressed(egui::Key::Num3) { Some(3) }
        else if i.key_pressed(egui::Key::Num4) { Some(4) }
        else if i.key_pressed(egui::Key::Num5) { Some(5) }
        else if i.key_pressed(egui::Key::Num6) { Some(6) }
        else if i.key_pressed(egui::Key::Num7) { Some(7) }
        else if i.key_pressed(egui::Key::Num8) { Some(8) }
        else if i.key_pressed(egui::Key::Num9) { Some(9) }
        else { None };

        if let Some(d) = d {
            grid[idx] = d;
        }
    });
}

fn clear_screen() {
    // ANSI escape code to clear terminal and move cursor to top-left
    print!("\n\n\n");
}

#[inline]
fn get_idx(row: usize, col: usize) -> usize {
    row * 9 + col
}

fn main() -> eframe::Result<()> {
    let args: Args = Args::parse();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 880.0])
            .with_min_inner_size([320.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native("Sudoku Solver", native_options, Box::new( |cc|
        Ok(Box::new(SudokuApp::new(cc, args)))
    ))
}
