use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    physics::Jump,
    player::{Player, Rock},
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KickEvent>()
            .register_type::<Kick>()
            .add_systems(Update, (kick, handle_kicks));
    }
}

#[derive(Event)]
pub struct KickEvent {
    origin: Vec2,
    force: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Kick {
    kick_strength: f32,
}

impl Kick {
    pub fn new(kick_strength: f32) -> Self {
        Kick { kick_strength }
    }
}

fn kick(
    rapier_context: Res<RapierContext>,
    mut kick_event: EventWriter<KickEvent>,
    input: Res<Input<KeyCode>>,
    query: Query<(&Transform, &Kick), With<Player>>,
) {
    for (transform, kick) in &query {
        // Where our raycast starts from
        let origin = transform.translation.xy();
        // How far the raycast travels
        let distance = 100.0;
        // Collision group stuff
        let collision_groups = CollisionGroups::new(
            Group::from_bits_truncate(0b0100),
            Group::from_bits_truncate(0b0100),
        );
        let filter = QueryFilter {
            groups: Some(collision_groups),
            ..default()
        };

        // if the rock is on our right or left side and we press e
        if (rapier_context
            .cast_ray(origin, Vec2::X, distance, true, filter)
            .is_some()
            || rapier_context
                .cast_ray(origin, Vec2::NEG_X, distance, true, filter)
                .is_some())
            && input.just_pressed(KeyCode::E)
        {
            kick_event.send(KickEvent {
                origin: transform.translation.xy(),
                force: kick.kick_strength,
            });
        }
    }
}

fn handle_kicks(
    // Enables us to read kick events
    mut kick_event: EventReader<KickEvent>,
    // Query for the rock
    mut query: Query<(&mut Velocity, &Transform), With<Rock>>,
) {
    // Loop through our query to find the rock
    // in the future we will probably react to the kick event with different objects as well
    for (mut velocity, transform) in &mut query {
        // for each entity we find, read any kick events
        for kick in kick_event.read() {
            // Get the direction that the kick should send the rock by subtracking the origin of
            // the kick from the rocks position
            let kick_direction = transform.translation.xy() - kick.origin;
            // apply the force of the kick along the normalized vector of the kicks direction
            let kick_force = kick_direction.normalize_or_zero() * kick.force;

            // add the force of the kick to the rock
            velocity.linvel += kick_force;
        }
    }
}
