use crate::combat::Kick;
use crate::physics::{GroundSensor, Jump};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, move_camera));
    }
}

#[derive(Resource)]
pub struct PlayerData {
    pub player_entity: Entity,
    pub rock_entity: Entity,
    pub terrain_collision_groups: CollisionGroups,
    pub player_collision_groups: CollisionGroups,
    pub rock_collision_groups: CollisionGroups,
}

impl PlayerData {
    pub fn new(
        player_entity: Entity,
        rock_entity: Entity,
        terrain_collision_groups: CollisionGroups,
        player_collision_groups: CollisionGroups,
        rock_collision_groups: CollisionGroups,
    ) -> Self {
        PlayerData {
            player_entity,
            rock_entity,
            terrain_collision_groups,
            player_collision_groups,
            rock_collision_groups,
        }
    }
}

#[derive(Resource)]
pub struct PlayerHealth {
    pub max: f32,
    pub current: f32,
}

// A marker component so that we can query the
#[derive(Component)]
pub struct MainCamera;

// A marker component so that we can query the player entity
#[derive(Component)]
pub struct Player;

// A marker component so that we can query the rock entity
#[derive(Component)]
pub struct Rock;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let terrain_groups = CollisionGroups::new(
        Group::from_bits_truncate(0b0001),
        Group::from_bits_truncate(0b0111),
    );
    let player_groups = CollisionGroups::new(
        Group::from_bits_truncate(0b0010),
        Group::from_bits_truncate(0b0101),
    );

    let rock_groups = CollisionGroups::new(
        Group::from_bits_truncate(0b0100),
        Group::from_bits_truncate(0b0111),
    );
    // Spawn our guy
    let player_entity = commands
        .spawn((
            SpriteBundle {
                // load the player sprite from the ./assets folder
                texture: asset_server.load("tower_guy.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::Y * 50.0),
                ..default()
            },
            Player,
            // Give the player a rigid body that can be moved
            RigidBody::Dynamic,
            // Give the player a capsule shaped collider
            Collider::capsule_y(30.0, 25.0),
            // Give the player a velocity component so that it can be moved by the physics system
            Velocity::default(),
            // Make it so that the player can not be rotated by physics interactions (so it doesn't
            // fall over while moveing)
            LockedAxes::ROTATION_LOCKED,
            // Give the player a jump component so that he can jump
            Jump::new(550.0),
            // Give the player a kick component so that he can kick
            Kick::new(1000.0),
            // Give the player a ground sensor so that it can detect that it's on the ground
            GroundSensor::default(),
            player_groups,
            Name::from("Player"),
        ))
        .id();

    let chain = RopeJointBuilder::new()
        .local_anchor1(Vec2::default())
        .local_anchor2(Vec2::default())
        .set_motor(50.0, 10.0, 10000.0, 1.0)
        .limits([0.0, 200.0])
        .build();

    let rock_entity = commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("rock.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(150., 150.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(200.0, 50.0, 0.0)),
                ..default()
            },
            Rock,
            RigidBody::Dynamic,
            Collider::ball(75.0),
            Velocity::default(),
            ImpulseJoint::new(player_entity, chain),
            rock_groups,
            Name::from("Rock"),
        ))
        .id();

    commands.insert_resource(PlayerData::new(
        player_entity,
        rock_entity,
        terrain_groups,
        player_groups,
        rock_groups,
    ));
}

fn move_player(
    // The input resource so we can check which keys are being pressed
    input: Res<Input<KeyCode>>,
    // The time resource so we can smooth our movement out, regardless of the frame rate
    time: Res<Time>,
    // Querying for the (External Force component, sprite component, gravity component) of
    // every entity with a (Player component)
    mut player_query: Query<(&mut Velocity, &mut Sprite), With<Player>>,
) {
    // Loop over the components of each entity we found with our query
    for (mut velocity, mut sprite) in &mut player_query {
        // create an x_speed variable to determine our horizontal movement
        let mut x_speed = 0.0;
        // Set our x_speed to move us to the right or left depending on which key is pressed, as
        // well as flip our sprite to face that direction
        if input.pressed(KeyCode::D) {
            x_speed = 1000.0 * time.delta_seconds();
            sprite.flip_x = false;
        }
        if input.pressed(KeyCode::A) {
            x_speed = -1000.0 * time.delta_seconds();
            sprite.flip_x = true;
        }

        //Add to the players velocity on the x-axis
        velocity.linvel += Vec2::X * x_speed;
    }
}

fn move_camera(
    // query for the (Transform component) of every entity with a (Main Camera Component)
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    // query for the (Transfrom component) of every entity with a (Player component) and without a
    // (Main Camera) component (for query mutability reasons)
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    // Set the cameras position to match the player
    camera_transform.translation = player_transform.translation;
}
