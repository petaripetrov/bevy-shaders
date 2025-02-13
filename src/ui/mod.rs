use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use bevy::{
    app::{Plugin, Update},
    log::{error, info, warn},
    prelude::{
        in_state, on_event, AppExtStates, EventReader, IntoSystemConfigs, NextState, Res, ResMut,
        Resource, State, StateTransitionEvent, States,
    },
    window::WindowCloseRequested,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::DemoState;

#[derive(Default, States, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RendererState {
    #[default]
    Basic,
    Toon,
    Pbr,
}

#[derive(Default, Resource, Serialize, Deserialize, Clone, Copy)]
pub struct MaterialSettings {
    pub color: [f32; 3],
}

#[derive(Default, Resource, Serialize, Deserialize, Clone, Copy)]
pub struct LightSettings {
    pub pos: [f32; 3],
    pub intensity: f32,
}

#[derive(Default, Serialize, Deserialize)]
struct UIState {
    renderer: RendererState,
    material: MaterialSettings,
    light: LightSettings,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Init EGUI
        app.add_plugins(EguiPlugin);
        let file = File::open("ui_state.json");

        if let Ok(f) = file {
            let reader = BufReader::new(f);
            let ui_state = serde_json::from_reader::<_, UIState>(reader);

            match ui_state {
                Ok(state) => {
                    app.insert_resource(state.material);
                    app.insert_resource(state.light);
                    app.insert_state(state.renderer);
                }
                Err(_) => {
                    warn!("Could not find UI State settings. Initializing with empty state");
                    app.init_resource::<MaterialSettings>();
                    app.init_resource::<LightSettings>();
                    app.init_state::<RendererState>();
                }
            };
        } else {
            warn!("Could not open UI state settings. Initiaizing with empty state");
            app.init_resource::<MaterialSettings>();
            app.init_resource::<LightSettings>();
            app.init_state::<RendererState>();
        }

        app.add_systems(
            Update,
            (
                spawn_ui.before(spawn_basic_ui),
                log_transitions,
                (spawn_light_ui, spawn_basic_ui.run_if(in_state(RendererState::Basic))).run_if(in_state(DemoState::Renderer)),
                save_ui_state.run_if(on_event::<WindowCloseRequested>),
            ),
        );
    }
}

// EguiContexts is an alias for a Query type
fn spawn_ui(
    mut egui_context: EguiContexts,
    demo_state: Res<State<DemoState>>,
    renderer_state: Res<State<RendererState>>,
    mut render_trans: ResMut<NextState<RendererState>>,
    mut demo_trans: ResMut<NextState<DemoState>>,
) {
    if let Some(context) = egui_context.try_ctx_mut() {
        // let mut curr_state = *renderer_state.get();

        let mut curr_state = *demo_state.get();

        egui::Window::new("Demo Controls")
            .vscroll(false)
            .resizable(true)
            .show(context, |ui| {

                egui::ComboBox::from_label("Demo")
                .selected_text(format!("{:?}", curr_state))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(
                            &mut curr_state,
                            DemoState::Renderer,
                            "Renderer",
                        )
                        .changed()
                    {
                        demo_trans.set(DemoState::Renderer);
                    }

                    if ui
                        .selectable_value(&mut curr_state, DemoState::Mapgen, "Mapgen")
                        .changed()
                    {
                        demo_trans.set(DemoState::Mapgen);
                    }
                });

                match curr_state {
                    DemoState::Renderer => {
                        let mut renderer_state = *renderer_state.get();
                        egui::ComboBox::from_label("Renderer")
                            .selected_text(format!("{:?}", renderer_state))
                            .show_ui(ui, |ui| {
                                if ui
                                    .selectable_value(
                                        &mut renderer_state,
                                        RendererState::Basic,
                                        "Basic",
                                    )
                                    .changed()
                                {
                                    render_trans.set(RendererState::Basic);
                                }

                                if ui
                                    .selectable_value(&mut renderer_state, RendererState::Toon, "Toon")
                                    .changed()
                                {
                                    render_trans.set(RendererState::Toon);
                                }

                                if ui
                                    .selectable_value(&mut renderer_state, RendererState::Pbr, "PBR")
                                    .changed()
                                {
                                    render_trans.set(RendererState::Pbr);
                                }
                            });
                    }
                    DemoState::Mapgen => {},
                }
            });
    }
}

fn spawn_basic_ui(mut egui_context: EguiContexts, mut material_settings: ResMut<MaterialSettings>) {
    if let Some(context) = egui_context.try_ctx_mut() {
        egui::Window::new("Basic Renderer")
            .vscroll(false)
            .resizable(true)
            .show(context, |ui| {
                ui.label("Material");

                ui.color_edit_button_rgb(&mut material_settings.as_mut().color);
            });
    }
}

fn spawn_light_ui(mut egui_context: EguiContexts, mut light: ResMut<LightSettings>) {
    if let Some(context) = egui_context.try_ctx_mut() {
        egui::Window::new("Light Controls")
            .vscroll(false)
            .resizable(true)
            .show(context, |ui| {
                ui.label("Light");

                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut light.pos[0]).speed(0.05));
                    ui.add(egui::DragValue::new(&mut light.pos[1]).speed(0.05));
                    ui.add(egui::DragValue::new(&mut light.pos[2]).speed(0.05));
                });

                ui.add(
                    egui::DragValue::new(&mut light.intensity)
                        .speed(0.05)
                        .range(0..=10),
                );
            });
    }
}

fn save_ui_state(
    material_settings: Res<MaterialSettings>,
    light: Res<LightSettings>,
    renderer_state: Res<State<RendererState>>,
) {
    let file = File::create("ui_state.json");
    let state = UIState {
        renderer: *renderer_state.get(), // rename to use the simple object builder

        material: *material_settings,
        light: *light,
    };

    if let Ok(f) = file {
        let mut writer = BufWriter::new(f);

        let write_res = serde_json::to_writer_pretty(&mut writer, &state);

        match write_res {
            Ok(_) => info!("Successfully saved state, exiting application."),
            Err(_) => error!("Could not write UI state to file. Aborting"),
        };
    } else {
        error!("Could not create or open UI State file.");
    }
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<RendererState>>) {
    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.exited, transition.entered
        );
    }
}
