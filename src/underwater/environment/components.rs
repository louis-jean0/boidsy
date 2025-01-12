use bevy::prelude::*;

#[derive(Component)]
pub struct Bubble {
    pub velocity: Vec3,
    pub lifetime: Timer,
}

#[derive(Resource)]
pub struct UnderwaterEffect {
    pub particle_spawn_timer: Timer,
    pub wave_animation_timer: Timer,
}

impl Default for UnderwaterEffect {
    fn default() -> Self {
        Self {
            particle_spawn_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            wave_animation_timer: Timer::from_seconds(0.016, TimerMode::Repeating),
        }
    }
}
