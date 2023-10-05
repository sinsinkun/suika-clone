use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::util::{
  AppState,
  // Score,
  SCREEN_H,
  CONTAINER_W,
  CONTAINER_H,
  CONTAINER_T,
  CONTAINER_P,
  CONTAINER_COLOR,
  BG_NO_MOVE_COLOR,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_game)
      .add_systems(Update, on_loop.run_if(in_state(AppState::InGame)))
      .add_systems(OnExit(AppState::InGame), pause_state);
  }
}

// -- COMPONENTS --
#[derive(Component)]
struct Cup;

// -- SYSTEMS --
fn setup_game(mut commands: Commands) {
  // initialize physics
  // setup initial state

  // spawn cup base with collider
  let container_base = -0.45 * SCREEN_H; // 5% offset from bottom
  commands.spawn((
    Cup,
    Collider::cuboid(CONTAINER_W / 2.0, CONTAINER_T / 2.0),
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(CONTAINER_W, CONTAINER_T)),
        color: CONTAINER_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(0.0, container_base, 0.0),
      ..default()
    },
  ));

  let wall_h = CONTAINER_H + CONTAINER_T;
  let wall_base = container_base + CONTAINER_H / 2.0;
  // spawn left wall
  commands.spawn((
    Cup,
    Collider::cuboid(CONTAINER_T / 2.0, wall_h / 2.0),
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(CONTAINER_T, wall_h)),
        color: CONTAINER_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(
        -CONTAINER_W / 2.0,
        wall_base,
        0.0,
      ),
      ..default()
    }
  ));
  // spawn right wall
  commands.spawn((
    Cup,
    Collider::cuboid(CONTAINER_T / 2.0, wall_h / 2.0),
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(CONTAINER_T, wall_h)),
        color: CONTAINER_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(
        CONTAINER_W / 2.0,
        wall_base,
        0.0,
      ),
      ..default()
    },
  ));

  // render unmovable zone left
  commands.spawn((
    Cup,
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(CONTAINER_P, wall_h)),
        color: BG_NO_MOVE_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(
        -(CONTAINER_W - CONTAINER_P) / 2.0,
        wall_base,
        0.0,
      ),
      ..default()
    },
  ));
  // render unmovable zone right
  commands.spawn((
    Cup,
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(CONTAINER_P, wall_h)),
        color: BG_NO_MOVE_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(
        (CONTAINER_W - CONTAINER_P) / 2.0,
        wall_base,
        0.0,
      ),
      ..default()
    },
  ));
  
  // reset score
}

fn on_loop() {
  // read inputs
  // calculate physics
  // calculate score
}

fn pause_state() {
  // pause physics
}