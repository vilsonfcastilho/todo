use crate::prelude::*;

pub struct TodoApp {
    pub tasks: Vec<(String, bool)>,
    pub input: String,
}

impl TodoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for TodoApp {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            input: String::new(),
        }
    }
}

impl App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("todo 📝")
                        .size(20.)
                        .color(egui::Color32::WHITE),
                );
            });

            // INPUT TEXT AND BUTTON SECTION
            ui.horizontal(|ui| {
                let input_response: egui::Response = ui.add_sized(
                    [ui.available_size().x - 25., 10.],
                    egui::TextEdit::singleline(&mut self.input).hint_text("What needs to be done?"),
                );

                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::RIGHT),
                    |ui: &mut egui::Ui| {
                        if ui.button(" + ").clicked() && !self.input.trim().is_empty() {
                            // Only add the task if input is not empty
                            self.tasks.push((self.input.clone(), false));
                            self.input.clear();

                            input_response.request_focus();
                        }
                    },
                );

                if ctx.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter)) {
                    let tasks: Vec<_> = self
                        .input
                        .lines()
                        .map(|line| (line.to_string(), false))
                        .collect();
                    self.tasks.extend(tasks);
                    self.input.clear();

                    input_response.request_focus();
                }
            });

            ui.add_space(10.);

            // BODY WITH THE TASK LIST
            egui::ScrollArea::vertical()
                .max_height(ui.available_height())
                .show(ui, |ui: &mut egui::Ui| {
                    ui.set_width(ui.available_size().x);

                    for (task, completed) in &mut self.tasks {
                        ui.horizontal(|ui: &mut egui::Ui| {
                            if *completed {
                                ui.add_enabled(false, egui::Checkbox::new(completed, ""));
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(task.clone()).strikethrough(),
                                    )
                                    .wrap(),
                                );
                            } else {
                                if ui.add(egui::Checkbox::new(completed, "")).clicked() {
                                    *completed = true; // Mark task as completed when checkbox is clicked
                                }
                                ui.add(egui::Label::new(egui::RichText::new(task.clone())).wrap());
                            }
                        });
                    }

                    if self.tasks.iter().all(|(_, completed)| *completed) {
                        // Verify if all the tasks is completed
                        ui.vertical_centered(|ui: &mut egui::Ui| {
                            ui.label("All tasks are completed!");
                        });

                        self.tasks.clear(); // Reset the application
                    }
                });
        });
    }
}
