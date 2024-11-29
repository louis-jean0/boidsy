use bevy::prelude::*;
use bevy_egui::*;

use crate::boids_2d::resources::BoidSettings;

pub fn setup_ui(
    mut egui_context: EguiContexts,
    mut boid_settings: ResMut<BoidSettings>) {
    egui::Window::new("Boids settings").show(egui_context.ctx_mut(), |ui| {
        let bounce = &mut boid_settings.bounce_against_walls;
        ui.checkbox(bounce, "Boids bounce against walls");
        let boids_count = &mut boid_settings.count;
        ui.add(egui::Slider::new(boids_count, 0..=2500).text("Boids count"));
        let min_speed = &mut boid_settings.min_speed;
        ui.add(egui::Slider::new(min_speed, 0.0..=500.0).text("Min speed"));
        let max_speed = &mut boid_settings.max_speed;
        ui.add(egui::Slider::new(max_speed, 0.0..=1000.0).text("Max speed"));
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
        let attraction_coeff = &mut boid_settings.attraction_coeff;
        ui.add(egui::Slider::new(attraction_coeff, 0.0..=100.0).text("Attraction to target"));
    });
}

pub fn show_fps(mut egui_context: EguiContexts) {
    egui::Window::new("FPS").show(egui_context.ctx_mut(), |ui| {
        ui.label("Test");
        ui.add(egui::TextEdit::singleline(&mut "60"));
    });
}