use clap::Parser;
use eframe::egui;

use sudoku_solver::ui::SudokuApp;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> eframe::Result<()> {
    let _args: Args = Args::parse();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 880.0])
            .with_min_inner_size([320.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native("Sudoku Solver", native_options, Box::new(|cc| {
        Ok(Box::new(SudokuApp::new(cc)))
    }))
}
