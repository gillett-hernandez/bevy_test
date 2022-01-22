use bevy::prelude::*;
use bevy_egui::*;

pub struct SimpleEgui;

impl Plugin for SimpleEgui {
    fn build(&self, app: &mut App) {
        app.add_system(simple).add_plugin(EguiPlugin);
    }
}

pub fn simple(egui: Res<EguiContext>) {
    egui::Window::new("Hello").show(egui.ctx(), |ui| {
        ui.label("world");
    });
}
