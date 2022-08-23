use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod settings;
mod orbs;

use crate::settings::*;
use crate::orbs::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Cellular Orbs".to_string(),
            width: 1280.0,
            height: 960.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(Settings::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(OrbPlugin)
        .add_startup_system(setup)
        .add_system(settings_ui)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
}