use eframe::egui;

pub fn draw_sudoku_grid(
    ui: &mut egui::Ui,
    grid: &[u8; 81],
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
        .spacing(egui::vec2(outer_gap, outer_gap))
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
                                        let resp = cell_button(ui, cell_size, label, is_selected);

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
    let interact = ui.style().interact(&resp);
    let visuals = ui.visuals();

    let (fill, stroke) = if selected {
        (visuals.selection.bg_fill, visuals.selection.stroke)
    } else {
        (interact.bg_fill, interact.bg_stroke)
    };

    ui.painter().rect(rect, 2.0, fill, stroke, egui::StrokeKind::Middle);

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
