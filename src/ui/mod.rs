use std::{fs::File, io::{BufReader, BufWriter}, ops::Deref};
use serde::{Deserialize, Serialize};

use bevy::{
    app::{Plugin, Update}, log::{error, info, warn}, prelude::{on_event, Commands, IntoSystemConfigs, Res, ResMut, Resource}, window::WindowCloseRequested
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum RendererState {
    #[default]
    Basic,
    Toon,
    PBR,
}

#[derive(Default, Serialize, Deserialize)]
struct MaterialSettings {
    color: [u8; 3],
}

#[derive(Resource, Default, Serialize, Deserialize)]
struct UIState {
    drag_control: f32,
    renderer: RendererState,
    material: MaterialSettings,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Init EGUI
        app.add_plugins(EguiPlugin);
        // app.state

        let file = File::open("ui_state.json");

        if let Ok(f) = file {
            let reader = BufReader::new(f);
            let ui_state = serde_json::from_reader::<_, UIState>(reader);

            match ui_state {
                Ok(state) => app.insert_resource(state),
                Err(_) => {
                    warn!("Could not fid UI State settings. Initializing with empty state");
                    app.init_resource::<UIState>()
                }
            };
        } else {
            warn!("Could not open UI state settings. Initiaizing with empty state");
            app.init_resource::<UIState>(); 
        }


        app.add_systems(
            Update,
            (
                spawn_ui,
                save_ui_state.run_if(on_event::<WindowCloseRequested>()),
            ),
        );
    }
}

// EguiContexts is an alias for a Query type
fn spawn_ui(mut egui_context: EguiContexts, mut _commands: Commands, mut egui_state: ResMut<UIState>) {
// TODO see if splitting up the UI can make sense. I.E. one function for the default UI, one for the Material, and one for each renderer.
    if let Some(context) = egui_context.try_ctx_mut() {
        egui::Window::new("Render Controls")
            .vscroll(false)
            .resizable(true)
            .show(context, |ui| {
                egui::ComboBox::from_label("Renderer")
                    .selected_text(format!("{:?}", egui_state.renderer))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut egui_state.renderer,
                            RendererState::Basic,
                            "Basic",
                        );
                        ui.selectable_value(&mut egui_state.renderer, RendererState::Toon, "Toon");
                        ui.selectable_value(&mut egui_state.renderer, RendererState::PBR, "PBR");
                    });

                match egui_state.renderer {
                    RendererState::Basic => {
                        ui.horizontal(|ui| {
                            ui.label("Kd value");
                            ui.color_edit_button_srgb(&mut egui_state.material.color);
                        });
                    }
                    RendererState::Toon => {}
                    RendererState::PBR => {}
                }

                ui.allocate_space(ui.available_size());
            });
    }
}

fn save_ui_state(state: Res<UIState>) {
    let file = File::create("ui_state.json");

    if let Ok(f) = file {
        let mut writer = BufWriter::new(f);

        let write_res = serde_json::to_writer_pretty(&mut writer, state.deref());

        match write_res {
            Ok(_) => info!("Successfully saved state, exiting application."),
            Err(_) => error!("Could not write UI state to file. Aborting")
        };
    } else {
        error!("Could not create or open UI State file.");
    }
}
