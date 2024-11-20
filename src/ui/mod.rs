use bevy::{
    app::{Plugin, Update},
    prelude::Commands,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default)]
struct EguiState {
    drag_control: f32,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Init EGUI
        app.add_plugins(EguiPlugin);

        let mut ui_state = EguiState { drag_control: 1.0 };

        app.add_systems(Update, move |ui: EguiContexts, cmd: Commands| {
            spawn_ui(ui, cmd, &mut ui_state)
        });
    }
}

// EguiContexts is an alias for a Query type
fn spawn_ui(mut egui_context: EguiContexts, mut _commands: Commands, egui_state: &mut EguiState) {
    if let Some(context) = egui_context.try_ctx_mut() {
        egui::Window::new("Render Controls")
            .vscroll(false)
            .resizable(true)
            // .default_size([250.0, 150.0])
            .show(context, |ui| {
                ui.label("Test");

                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut egui_state.drag_control)
                            .speed(1.0)
                            .range(-5.0..=5.0),
                    )
                });

                ui.allocate_space(ui.available_size());
            });
    }
}
