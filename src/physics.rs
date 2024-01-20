use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (jump, handle_ground_sensor))
            .register_type::<Jump>();
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
pub enum GroundedState {
    Grounded,
    #[default]
    Airborne,
}
#[derive(Component, Default)]
pub struct GroundSensor {
    state: GroundedState,
}

impl GroundSensor {
    pub fn grounded(&self) -> bool {
        self.state == GroundedState::Grounded
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Jump {
    jump_force: f32,
    double_jump_available: bool,
}

impl Jump {
    pub fn new(jump_force: f32) -> Self {
        Jump {
            jump_force,
            double_jump_available: true,
        }
    }
    pub fn land(&mut self) {
        self.double_jump_available = true;
    }

    pub fn jump_force(&self) -> f32 {
        self.jump_force
    }
}

fn handle_ground_sensor(
    rapier_context: Res<RapierContext>,
    mut query: Query<(Entity, &mut GroundSensor, &Transform)>,
) {
    // For every component that has a `GroundSensor`
    for (entity, mut ground_sensor, transform) in &mut query {
        // set the start of the raycast
        let origin = transform.translation.xy();
        // send it straight down because we are trying to detect the ground
        let dir = Vec2::NEG_Y;
        // cast for 1000 pixels
        let distance = 100.0;
        // Do not collide with the entity that is making this raycast
        let filter = QueryFilter::default().exclude_collider(entity);

        if let Some(_) = rapier_context.cast_ray(origin, dir, distance, true, filter) {
            // If we've hit ground, set our state to grounded
            ground_sensor.state = GroundedState::Grounded;
        } else {
            // If we haven't hit ground, set us to airborne
            ground_sensor.state = GroundedState::Airborne;
        }
    }
}

fn jump(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundSensor, &Jump), With<Player>>,
) {
    for (mut velocity, ground_sensor, jumper) in &mut query {
        // If the player is on solid ground and the space key was just pressed
        if input.just_pressed(KeyCode::Space) && ground_sensor.grounded() {
            // Add our jump force to our Y velocity
            velocity.linvel.y += jumper.jump_force();
        }
    }
}
