use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{physics::{Jump, GroundSensor}, combat::Kick};

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle{
  sprite_bundle: SpriteBundle,
  enemy: Enemy,
  rigidbody: RigidBody,
  collider: Collider,
  velocity: Velocity,
  locked_axes: LockedAxes,
  jump: Jump,
  kick: Kick,
  ground_sensor: GroundSensor,
  name: Name
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle{
          sprite_bundle: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        enemy: Enemy,
        rigidbody: RigidBody::Dynamic,
        // Give the player a capsule shaped collider
        collider: Collider::capsule_y(30.0, 25.0),
        // Give the player a velocity component so that it can be moved by the physics system
        velocity: Velocity::default(),
        // Make it so that the player can not be rotated by physics interactions (so it doesn't
        // fall over while moveing)
        locked_axes: LockedAxes::ROTATION_LOCKED,
        // Give the player a jump component so that he can jump
        jump: Jump::new(550.0),
        // Give the player a kick component so that he can kick
        kick: Kick::new(1000.0),
        // Give the player a ground sensor so that it can detect that it's on the ground
        ground_sensor: GroundSensor::default(),
        name: Name::from("Enemy")
        }
    }
}

// TODO: Make genric when it makes sense
#[derive(Resource)]
pub struct EnemySpawner{
    count: u8,
    spawn_time: Timer
}

impl EnemySpawner {
    pub fn tick(&mut self, delta: std::time::Duration){
      self.spawn_time.tick(delta);
    }

    pub fn should_spawn(&self) -> bool{
      self.spawn_time.just_finished()
    }

    pub fn spawn_enemy(&mut self) {
        self.count+=1;
        self.spawn_time.reset();
    }
}
// TODO decide spawn location
fn spawn_enemy(mut commands: Commands, mut spawner: ResMut<EnemySpawner>, time: Res<Time>, asset_server: Res<AssetServer>) {
  spawner.tick(time.delta());
  if spawner.should_spawn(){
    spawner.spawn_enemy();
    // TODO fix Colins BUll SHIT
    commands.spawn(
      EnemyBundle{
        sprite_bundle: SpriteBundle{
          texture: asset_server.load("tower_guy.png"), 
          sprite: Sprite{
            color: Color::BLUE,
            custom_size: Some(Vec2::new(100., 100.)),
            ..Default::default()
          }, 
          ..Default::default()
        },
        ..Default::default()
      });
  }
}