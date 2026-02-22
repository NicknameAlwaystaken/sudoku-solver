use eframe::egui;

pub fn handle_digit_keys(ctx: &egui::Context, grid: &mut [u8; 81], selected: Option<usize>, givens: &[bool; 81]) {
    let Some(idx) = selected else { return };
    if givens[idx] { return; } // lock givens

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
