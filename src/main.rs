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
        // Set the strength of our gravity
        .insert_resource(RapierConfiguration {
            gravity: Vec2::NEG_Y * 1000.0,
            ..default()
        })
        .register_type::<Kick>()
        .register_type::<Jump>()
        // Add our `setup` system to the Startup stage, so that it runs one time when the app
        // starts up
        .add_systems(Startup, setup)
        // Add the rest of our systems to the Update stage so that they run every tick/frame that
        // the application is running
        .add_systems(
            Update,
            (move_player, move_camera, handle_ground_sensor, jump),
        )
        .run();
}

// A marker component so that we can query the player entity
#[derive(Component)]
pub struct Player;

// A marker component so that we can query the rock entity
#[derive(Component)]
pub struct Rock;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Kick {
    kick_strength: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Jump {
    jump_force: f32,
    double_jump_available: bool,
}

impl Jump {
    pub fn land(&mut self) {
        self.double_jump_available = true;
    }

    pub fn jump_force(&self) -> f32 {
        self.jump_force
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
    for i in 0..50 {
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

    commands.spawn((
        SpriteBundle {
            // Load the block sprite from out ./assets folder
            texture: asset_server.load("Block_basic.png"),
            sprite: Sprite {
                // Set the sprite to be displayed at 100x100 pixels instead of it's native
                // resolution
                custom_size: Some(Vec2::new(500., 100.)),
                ..default()
            },
            // Set the position for the block
            transform: Transform::from_translation(Vec3::new(-1000.0, -100.0, 0.0))
                .with_rotation(Quat::from_axis_angle(Vec3::Z, 30.0_f32.to_degrees())),
            ..default()
        },
        // Give the block a collider so that it can be stood on
        Collider::cuboid(250.0, 50.0),
        // Give the block a fixed rigid body, meaning it won't move but can be collided with
        RigidBody::Fixed,
        Name::from("Ramp"),
    ));

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
            Collider::capsule_y(30.0, 25.0),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
            Jump {
                jump_force: 550.0,
                ..default()
            },
            GroundSensor::default(),
            Name::from("Player"),
        ))
        .id();

    let joint = RopeJointBuilder::new()
        .local_anchor1(Vec2::default())
        .local_anchor2(Vec2::default())
        .limits([0.0, 200.0]);

    commands.spawn((
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
        ImpulseJoint::new(player_entity, joint),
        Name::from("Rock"),
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

fn handle_ground_sensor(
    rapier_context: Res<RapierContext>,
    mut query: Query<(Entity, &mut GroundSensor, &Transform)>,
) {
    for (entity, mut ground_sensor, transform) in &mut query {
        let origin = transform.translation.xy();
        let dir = Vec2::NEG_Y;
        let distance = 100.0;
        let filter = QueryFilter::default().exclude_collider(entity);

        if let Some(_) = rapier_context.cast_ray(origin, dir, distance, true, filter) {
            ground_sensor.state = GroundedState::Grounded;
        } else {
            ground_sensor.state = GroundedState::Airborne;
        }
    }
}

fn jump(input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &GroundSensor, &Jump)>) {
    for (mut velocity, ground_sensor, jumper) in &mut query {
        if input.just_pressed(KeyCode::Space) && ground_sensor.grounded() {
            velocity.linvel.y += jumper.jump_force();
        }
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
    camera_transform.translation = player_transform.translation;
}
