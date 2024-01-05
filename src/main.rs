use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{na::distance_squared, prelude::*};

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
        .add_event::<KickEvent>()
        .register_type::<Kick>()
        .register_type::<Jump>()
        // Add our `setup` system to the Startup stage, so that it runs one time when the app
        // starts up
        .add_systems(Startup, setup)
        // Add the rest of our systems to the Update stage so that they run every tick/frame that
        // the application is running
        .add_systems(
            Update,
            (
                move_player,
                move_camera,
                handle_ground_sensor,
                jump,
                kick,
                handle_kicks,
            ),
        )
        .run();
}

#[derive(Resource)]
pub struct PlayerData {
    pub player_entity: Entity,
    pub rock_entity: Entity,
}

impl PlayerData {
    pub fn new(player_entity: Entity, rock_entity: Entity) -> Self {
        PlayerData {
            player_entity,
            rock_entity,
        }
    }
}

#[derive(Event)]
pub struct KickEvent {
    origin: Vec2,
    force: f32,
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

impl Kick {
    pub fn new(kick_strength: f32) -> Self {
        Kick { kick_strength }
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

    // 0  Ground
    // 1 Player
    // 2 Rock
    //
    // 0, [1,2]
    // 1, [0,2]
    //

    let terrain_group = Group::GROUP_1;
    let terrain_filter = Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3;

    let player_group = Group::GROUP_2;
    let player_filter = Group::GROUP_1 | Group::GROUP_3;

    let rock_group = Group::GROUP_3;
    let rock_filter = Group::GROUP_1 | Group::GROUP_2;

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
            CollisionGroups::new(terrain_group, terrain_filter),
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
        CollisionGroups::new(terrain_group, terrain_filter),
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
            Jump::new(550.0),
            Kick::new(1000.0),
            GroundSensor::default(),
            CollisionGroups::new(player_group, player_filter),
            Name::from("Player"),
        ))
        .id();

    let joint = RopeJointBuilder::new()
        .local_anchor1(Vec2::default())
        .local_anchor2(Vec2::default())
        .limits([0.0, 200.0]);

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
            ImpulseJoint::new(player_entity, joint),
            CollisionGroups::new(rock_group, rock_filter),
            Name::from("Rock"),
        ))
        .id();

    commands.insert_resource(PlayerData::new(player_entity, rock_entity));
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

fn jump(input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &GroundSensor, &Jump)>) {
    for (mut velocity, ground_sensor, jumper) in &mut query {
        // If the player is on solid ground and the space key was just pressed
        if input.just_pressed(KeyCode::Space) && ground_sensor.grounded() {
            // Add our jump force to our Y velocity
            velocity.linvel.y += jumper.jump_force();
        }
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
        let distance = 1000.0;
        // Collision group stuff
        let collision_groups = CollisionGroups {
            memberships: Group::GROUP_2,
            filters: Group::GROUP_3,
        };
        let filter = QueryFilter {
            groups: Some(collision_groups),
            ..default()
        };

        // Send one ray to the right
        if rapier_context
            .cast_ray(origin, Vec2::X, distance, true, QueryFilter::default())
            .is_some()
            // Send another ray to the left
            && rapier_context
                .cast_ray(origin, Vec2::NEG_X, distance, true, QueryFilter::default())
                .is_some()
            // Check if the E key was pressed this frame
            && input.just_pressed(KeyCode::E)
        {
            // Send a kick event, with the origin as the players position and the force comes from
            // the players `Kick` component
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
