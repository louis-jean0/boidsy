use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::*;

use crate::boids_2d::resources::BoidSettings;
use crate::input::resources::ShapeSettings;
use crate::ui::events::CursorVisibilityEvent;

//fps
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

#[derive(Component)]
pub struct FpsRoot;

#[derive(Component)]
pub struct FpsText;

pub fn setup_ui(
    mut egui_context: EguiContexts,
    mut boid_settings: ResMut<BoidSettings>,
    mut shape_settings: ResMut<ShapeSettings>) {
    egui::Window::new("Boids settings").show(egui_context.ctx_mut(), |ui| {
        let bounce = &mut boid_settings.bounce_against_walls;
        ui.checkbox(bounce, "Boids bounce against walls");
        let boids_count = &mut boid_settings.count;
        ui.add(egui::Slider::new(boids_count, 0..=15000).text("Boids count"));
        let min_speed = &mut boid_settings.min_speed;
        ui.add(egui::Slider::new(min_speed, 0.0..=500.0).text("Min speed"));
        let max_speed = &mut boid_settings.max_speed;
        ui.add(egui::Slider::new(max_speed, 0.0..=1000.0).text("Max speed"));
        let field_of_view = &mut boid_settings.field_of_view;
        ui.add(egui::Slider::new(field_of_view, 0.0..=360.0).text("Field of view"));
        let cohesion_range = &mut boid_settings.cohesion_range;
        ui.add(egui::Slider::new(cohesion_range, 0.0..=100.0).text("Cohesion range"));
        let max_alignment_range = *cohesion_range;
        let alignment_range = &mut boid_settings.alignment_range;
        ui.add(egui::Slider::new(alignment_range, 0.0..=max_alignment_range).text("Alignment range"));
        let max_separation_range = *alignment_range;
        let separation_range = &mut boid_settings.separation_range;
        ui.add(egui::Slider::new(separation_range, 0.0..=max_separation_range).text("Separation range"));
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
        let radius = &mut shape_settings.radius;
        ui.add(egui::Slider::new(radius,1.0..=100.0).text("Radius of obstacles"));
        ui.label("R to remove all obstacles");
    });
}

pub fn handle_cursor_visibility(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_events: EventReader<CursorVisibilityEvent>
) {
    println!("Handling cursor visibility");
    for CursorVisibilityEvent{visible} in cursor_events.read() {
        if let Ok(mut window) = window_query.get_single_mut() {
            window.cursor.visible = *visible;
        }
    }
}

pub fn setup_fps_counter(
    mut commands: Commands
) {
    let root = commands.spawn((
        FpsRoot,
        NodeBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.),
                top: Val::Percent(1.),
                bottom: Val::Auto,
                left: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )).id();
    let text_fps = commands.spawn((
        FpsText,
        TextBundle {
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
                TextSection {
                    value: " N/A".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    }
                },
            ]),
            ..Default::default()
        },
    )).id();
    commands.entity(root).push_children(&[text_fps]);
}

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections[1].value = format!("{value:>4.0}");
            text.sections[1].style.color = if value >= 120.0 {
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                Color::rgb(
                    (1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
                    1.0,
                    0.0,
                )
            } else if value >= 30.0 {
                Color::rgb(
                    1.0,
                    ((value - 30.0) / (60.0 - 30.0)) as f32,
                    0.0,
                )
            } else {
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }

    }
}

/// Toggle the FPS counter when pressing F12
pub fn fps_counter_showhide(
    mut q: Query<&mut Visibility, With<FpsRoot>>,
    kbd: Res<Input<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}