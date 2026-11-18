//! Interface gráfica com egui (immediate mode).
//!
//! A cada frame o egui chama `update`: descrevemos os botões e o display,
//! e reagimos aos cliques. Não há “árvore de widgets” persistente como em Qt.

use eframe::egui::{self, Color32, FontId, RichText, Sense, Vec2};

use crate::calculator::{Calculator, Operation};

pub struct CalculatorApp {
    calc: Calculator,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            calc: Calculator::new(),
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(
                    RichText::new("rustyCalculator")
                        .font(FontId::proportional(18.0))
                        .color(Color32::from_rgb(160, 170, 190)),
                );
                ui.add_space(6.0);
            });

            // Display
            let display = format_display(self.calc.display(), self.calc.pending_op());
            egui::Frame::NONE
                .fill(Color32::from_rgb(28, 32, 40))
                .inner_margin(egui::Margin::symmetric(16, 12))
                .corner_radius(8.0)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(display)
                                .font(FontId::monospace(28.0))
                                .color(Color32::from_rgb(236, 240, 245)),
                        );
                    });
                });

            ui.add_space(12.0);

            let button_size = Vec2::new(72.0, 52.0);
            let gap = 8.0;

            // Layout: C  ±  ⌫  ÷
            //         7  8  9  ×
            //         4  5  6  −
            //         1  2  3  +
            //         0  .  =
            let rows: [&[&str]; 5] = [
                &["C", "±", "⌫", "÷"],
                &["7", "8", "9", "×"],
                &["4", "5", "6", "−"],
                &["1", "2", "3", "+"],
                &["0", ".", "="],
            ];

            for row in rows {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    for label in row {
                        let size = if *label == "0" {
                            Vec2::new(button_size.x * 2.0 + gap, button_size.y)
                        } else {
                            button_size
                        };

                        if calc_button(ui, label, size).clicked() {
                            self.handle_key(label);
                        }
                    }
                });
                ui.add_space(gap);
            }

            // Atalhos de teclado
            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } = event
                    {
                        match key {
                            egui::Key::Num0 | egui::Key::Numpad0 => self.calc.input_digit('0'),
                            egui::Key::Num1 | egui::Key::Numpad1 => self.calc.input_digit('1'),
                            egui::Key::Num2 | egui::Key::Numpad2 => self.calc.input_digit('2'),
                            egui::Key::Num3 | egui::Key::Numpad3 => self.calc.input_digit('3'),
                            egui::Key::Num4 | egui::Key::Numpad4 => self.calc.input_digit('4'),
                            egui::Key::Num5 | egui::Key::Numpad5 => self.calc.input_digit('5'),
                            egui::Key::Num6 | egui::Key::Numpad6 => self.calc.input_digit('6'),
                            egui::Key::Num7 | egui::Key::Numpad7 => self.calc.input_digit('7'),
                            egui::Key::Num8 | egui::Key::Numpad8 => self.calc.input_digit('8'),
                            egui::Key::Num9 | egui::Key::Numpad9 => self.calc.input_digit('9'),
                            egui::Key::Period | egui::Key::NumpadDecimal => {
                                self.calc.input_decimal()
                            }
                            egui::Key::Enter | egui::Key::NumpadEnter => self.calc.evaluate(),
                            egui::Key::Backspace => self.calc.backspace(),
                            egui::Key::Escape => self.calc.clear_all(),
                            egui::Key::Plus | egui::Key::NumpadAdd => {
                                self.calc.set_operation(Operation::Add)
                            }
                            egui::Key::Minus | egui::Key::NumpadSubtract => {
                                self.calc.set_operation(Operation::Subtract)
                            }
                            egui::Key::NumpadMultiply => {
                                self.calc.set_operation(Operation::Multiply)
                            }
                            egui::Key::NumpadDivide | egui::Key::Slash
                                if !modifiers.shift =>
                            {
                                self.calc.set_operation(Operation::Divide)
                            }
                            _ => {}
                        }
                    }
                }
            });
        });
    }
}

impl CalculatorApp {
    fn handle_key(&mut self, label: &str) {
        match label {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                self.calc.input_digit(label.chars().next().unwrap());
            }
            "." => self.calc.input_decimal(),
            "+" => self.calc.set_operation(Operation::Add),
            "−" => self.calc.set_operation(Operation::Subtract),
            "×" => self.calc.set_operation(Operation::Multiply),
            "÷" => self.calc.set_operation(Operation::Divide),
            "=" => self.calc.evaluate(),
            "C" => self.calc.clear_all(),
            "⌫" => self.calc.backspace(),
            "±" => self.calc.toggle_sign(),
            _ => {}
        }
    }
}

fn format_display(value: &str, pending: Option<Operation>) -> String {
    match pending {
        Some(op) if !value.starts_with("Divisão") && !value.starts_with("Número") => {
            format!("{} {}", op.symbol(), value)
        }
        _ => value.to_string(),
    }
}

fn calc_button(ui: &mut egui::Ui, label: &str, size: Vec2) -> egui::Response {
    let (fill, text_color) = button_colors(label);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let fill = if response.hovered() {
            lighten(fill, 18)
        } else {
            fill
        };
        ui.painter().rect_filled(rect, 8.0, fill);
        if response.hovered() {
            ui.painter()
                .rect_stroke(rect, 8.0, (1.0, visuals.fg_stroke.color), egui::StrokeKind::Outside);
        }
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            FontId::proportional(20.0),
            text_color,
        );
    }

    response
}

fn button_colors(label: &str) -> (Color32, Color32) {
    match label {
        "÷" | "×" | "−" | "+" | "=" => (
            Color32::from_rgb(52, 120, 246),
            Color32::WHITE,
        ),
        "C" | "±" | "⌫" => (
            Color32::from_rgb(70, 78, 94),
            Color32::from_rgb(230, 235, 245),
        ),
        _ => (
            Color32::from_rgb(45, 52, 64),
            Color32::from_rgb(230, 235, 245),
        ),
    }
}

fn lighten(c: Color32, amount: u8) -> Color32 {
    Color32::from_rgb(
        c.r().saturating_add(amount),
        c.g().saturating_add(amount),
        c.b().saturating_add(amount),
    )
}
