use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        // Add the default plugins, basically you will do this for 100% of bevy projects
        .add_plugins(DefaultPlugins)
        // Add our physics plugin
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // Add a debug plugin that renders our physics colliders
        .add_plugins(RapierDebugRenderPlugin::default())
        // A debug plugin that allows us to view the components attached to each entity
        .add_plugins(WorldInspectorPlugin::default())
        // Add our `setup` system to the Startup stage, so that it runs one time when the app
        // starts up
        .add_systems(Startup, setup)
        // Add the rest of our systems to the Update stage so that they run every tick/frame that
        // the application is running
        .add_systems(Update, (move_player, move_camera, add_gravity))
        .run();
}

// A marker component so that we can query the player entity
#[derive(Component)]
pub struct Player;

// A marker component so that we can query the
#[derive(Component)]
pub struct MainCamera;

// A component to apply gravity to an entity
#[derive(Component, Default)]
pub struct Gravity(f32);

impl Gravity {
    // Create a new gravity component
    pub fn new(value: f32) -> Self {
        Gravity(value)
    }

    // Add force to a gravity component
    pub fn add(&mut self, value: f32) {
        self.0 += value;
    }

    // Reset the the gravity component
    pub fn reset(&mut self) {
        self.0 = 0.0;
    }

    // Get the current force for a gravity component
    pub fn get(&self) -> f32 {
        self.0
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Starting position for the platforms
    let starting_translation = Vec3::new(-150.0, -120.0, 0.0);

    // Instead of copy pasting the platform spawn a bunch we just use a for loop here
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                // Load the block sprite from out ./assets folder
                texture: asset_server.load("Block_basic.png"),
                sprite: Sprite {
                    // Set the sprite to be displayed at 100x100 pixels instead of it's native
                    // resolution
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..default()
                },
                // Set the position for the block
                transform: Transform::from_translation(
                    starting_translation + (Vec3::X * (i * 100) as f32),
                ),
                ..default()
            },
            // Give the block a collider so that it can be stood on
            Collider::cuboid(50.0, 50.0),
            // Give the block a fixed rigid body, meaning it won't move but can be collided with
            RigidBody::Fixed,
        ));
    }

    // Spawn our guy
    commands.spawn((
        SpriteBundle {
            // load the player sprite from the ./assets folder
            texture: asset_server.load("tower_guy.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        Player,
        // Give the player a rigid body that can be moved
        RigidBody::KinematicPositionBased,
        Collider::ball(25.0),
        // Give the character a character controller so it can be moved more easily and detect
        // whether it's on the ground or not
        KinematicCharacterController::default(),
        // A component to move the player downward every frame that it's off the ground
        Gravity::default(),
    ));
}

fn move_player(
    // The input resource so we can check which keys are being pressed
    input: Res<Input<KeyCode>>,
    // The time resource so we can smooth our movement out, regardless of the frame rate
    time: Res<Time>,
    // Querying for the (Character Controller component, sprite component, gravity component) of
    // every entity with a (Player component)
    mut player_query: Query<
        (&mut KinematicCharacterController, &mut Sprite, &Gravity),
        With<Player>,
    >,
) {
    // Loop over the components of each entity we found with our query
    for (mut controller, mut sprite, gravity) in &mut player_query {
        // create an x_speed variable to determine our horizontal movement
        let mut x_speed = 0.0;
        // Set our x_speed to move us to the right or left depending on which key is pressed, as
        // well as flip our sprite to face that direction
        if input.pressed(KeyCode::D) {
            x_speed = 100.0 * time.delta_seconds();
            sprite.flip_x = false;
        }
        if input.pressed(KeyCode::A) {
            x_speed = -100.0 * time.delta_seconds();
            sprite.flip_x = true;
        }

        let movement_vector = Vec2::X * x_speed;
        // add our gravity to the speed value we are going to use to move our character
        let gravity_vector = Vec2::NEG_Y * gravity.get();
        println!("gravity value:{}", gravity.get());

        // Apply that vector to our character controller
        controller.translation = Some(movement_vector + gravity_vector);
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
    // Set the cameras x position to match the player
    camera_transform.translation.x = player_transform.translation.x;
}

fn add_gravity(
    // Grab the time resource to apply gravity evenly
    time: Res<Time>,
    // Grab the (Gravity component, Character controller output) of any entity that has all of
    // those components
    mut controller_query: Query<(&mut Gravity, &KinematicCharacterControllerOutput)>,
) {
    for (mut gravity, output) in &mut controller_query {
        // If we are on the ground
        if output.grounded {
            // Reset our gravity value to 0.0
            gravity.reset();
        } else {
            // if we are in the air add to our gravity value
            gravity.add(5.0 * time.delta_seconds());
        }
    }
}
