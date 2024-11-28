use bevy::prelude::*;
use bevy_egui::*;

use crate::boids_2d::resources::BoidSettings;

pub fn setup_ui(
    mut egui_context: EguiContexts,
    commands: Commands,
    mut boid_settings: ResMut<BoidSettings>) {
    egui::Window::new("Boids settings").show(egui_context.ctx_mut(), |ui| {
        let bounce = &mut boid_settings.bounce_against_walls;
        ui.checkbox(bounce, "Boids bounce against walls");
        let boids_count = &mut boid_settings.count;
        ui.add(egui::Slider::new(boids_count, 0..=1000).text("Boids count"));
        let alignment_range = &mut boid_settings.alignment_range;
        ui.add(egui::Slider::new(alignment_range, 0.0..=100.0).text("Alignment range"));
        let cohesion_range = &mut boid_settings.cohesion_range;
        ui.add(egui::Slider::new(cohesion_range, 0.0..=75.0).text("Cohesion range"));
        let separation_range = &mut boid_settings.separation_range;
        ui.add(egui::Slider::new(separation_range, 0.0..=50.0).text("Separation range"));
        let cohesion_coeff = &mut boid_settings.cohesion_coeff;
        ui.add(egui::Slider::new(cohesion_coeff, 0.0..=50.0).text("Cohesion"));
        let aligment_coeff = &mut boid_settings.alignment_coeff;
        ui.add(egui::Slider::new(aligment_coeff, 0.0..=50.0).text("Alignment"));
        let separation_coeff = &mut boid_settings.separation_coeff;
        ui.add(egui::Slider::new(separation_coeff, 0.0..=50.0).text("Separation"));
        let min_distance_between_boids = &mut boid_settings.min_distance_between_boids;
        ui.add(egui::Slider::new(min_distance_between_boids, 0.0..=50.0).text("Minimum distance between boids"));
        let collision_coeff = &mut boid_settings.collision_coeff;
        ui.add(egui::Slider::new(collision_coeff, 0.0..=50.0).text("Collision"));
    });
}