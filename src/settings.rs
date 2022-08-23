use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContext,
};
pub struct Settings {
    pub wraparound: bool,
    pub G: f32,
    pub friction: f32,
    pub influence_distance: f32,
    pub scale: f32,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            G: 100.,
            wraparound: false,
            friction: 0.1,
            influence_distance: 100.,
            scale: 1.,
        }
    }
}

pub fn settings_ui(
    mut egui_context: ResMut<EguiContext>,
    mut settings: ResMut<Settings>,
) {
    egui::Window::new("Configurations").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Play Field");
        ui.checkbox(&mut settings.wraparound, "Wraparound");
        ui.heading("Simulation");
        ui.add(egui::Slider::new(&mut settings.G, 0. ..=1000.).text("G"));
        ui.add(egui::Slider::new(&mut settings.friction, 0. ..=1.).text("Friction"));
        ui.add(egui::Slider::new(&mut settings.influence_distance, 0. ..=1000.).text("Influence Distance"));
        ui.add(egui::Slider::new(&mut settings.scale, 0.001..=1.).text("Scale"));
    });
}