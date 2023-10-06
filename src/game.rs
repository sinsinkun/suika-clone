use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::util::{
  AppState,
  Score,
  SpawnedFruit,
  SCREEN_H,
  CONTAINER_W,
  CONTAINER_H,
  CONTAINER_T,
  CONTAINER_P,
  CONTAINER_COLOR,
  BG_NO_MOVE_COLOR,
  SUIKA,
  HOLD_POS,
  HOLD_POS_FRUIT,
  MOVE_SPEED,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), setup_game)
      .add_systems(Update, end_game.run_if(in_state(AppState::InGame)))
      .add_systems(Update, handle_active_fruit.run_if(in_state(AppState::InGame)))
      .add_systems(Update, handle_next_fruit.run_if(in_state(AppState::InGame)))
      .add_systems(OnExit(AppState::InGame), pause_state);
  }
}

// -- COMPONENTS --
#[derive(Component)]
struct UIComponent;

#[derive(Component)]
struct Cup;

#[derive(Component)]
struct ActiveFruit(i32);

#[derive(Component)]
struct NextFruit(i32);

#[derive(Component)]
struct PreviewBar;

// -- SYSTEMS --
fn setup_game(
  mut commands: Commands, 
  mut score_q: Query<&mut Score>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
  
  // render hold area
  commands.spawn((
    UIComponent,
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Quad::new(Vec2::new(120.0, 120.0)).into()).into(),
      material: materials.add(ColorMaterial::from(BG_NO_MOVE_COLOR)),
      transform: Transform::from_translation(HOLD_POS),
      ..default()
    }
  ));

  // reset current score
  match score_q.get_single_mut() {
    Ok(mut score) => {
      score.0 = 0;
    },
    Err(_) => println!("Could not find score")
  }

}

fn end_game(
  mut next_state: ResMut<NextState<AppState>>,
  keys: Res<Input<KeyCode>>,
) {
  if keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::Escape) {
    next_state.set(AppState::GameOver);
  }
}

fn handle_active_fruit(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut active_fruit_q: Query<(Entity, &mut Transform), With<ActiveFruit>>,
  keys: Res<Input<KeyCode>>,
) {
  // spawn active fruit if not exist
  match active_fruit_q.get_single_mut() {
    Ok((_entity, transform)) => {
      // move active fruit
      let mut move_dir = 0.0;
      if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
        move_dir -= 1.0;
      }
      if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
        move_dir += 1.0;
      }
      // update active fruit render
      let new_x = transform.clone().translation.x + MOVE_SPEED * move_dir;
      let limit = CONTAINER_W / 2.0 - CONTAINER_P;
      if new_x > -limit && new_x < limit {
        transform.into_inner().translation.x = new_x;
      } else if new_x > limit {
        transform.into_inner().translation.x = -limit;
      } else if new_x < -limit {
        transform.into_inner().translation.x = limit;
      }
    },
    Err(_) => {
      // pick random fruit
      let num: i32 = rand::thread_rng().gen_range(0..5);
      let active_fruit = SUIKA[num as usize];

      // preview sizing
      let p_height = 1.25 * CONTAINER_H;
      let p_offset = -(0.5 * CONTAINER_H + 50.0);

      // spawn new active fruit
      commands.spawn((
        ActiveFruit(num),
        SpawnedFruit,
        MaterialMesh2dBundle {
          mesh: meshes.add(shape::Circle::new(active_fruit.size / 2.0).into()).into(),
          material: materials.add(ColorMaterial::from(active_fruit.color)),
          transform: Transform::from_translation(Vec3::new(0.0, CONTAINER_H / 2.0, 0.5)),
          ..default()
        },
      )).with_children(|root| {
        // spawn preview bar
        root.spawn(MaterialMesh2dBundle {
          mesh: meshes.add(shape::Quad::new(Vec2::new(1.5, p_height)).into()).into(),
          material: materials.add(ColorMaterial::from(Color::WHITE)),
          transform: Transform::from_translation(Vec3::new(0.0, p_offset, 1.0)),
          ..default()
        });
      });
    }
  }
}

fn handle_next_fruit(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut next_fruit_q: Query<&mut NextFruit>,
) {
  // spawn active fruit if not exist
  match next_fruit_q.get_single_mut() {
    Ok(_) => (),
    Err(_) => {
      // pick random fruit
      let num: i32 = rand::thread_rng().gen_range(0..5);
      let next_fruit = SUIKA[num as usize];

      // spawn new active fruit
      commands.spawn((
        NextFruit(num),
        SpawnedFruit,
        MaterialMesh2dBundle {
          mesh: meshes.add(shape::Circle::new(next_fruit.size / 2.0).into()).into(),
          material: materials.add(ColorMaterial::from(next_fruit.color)),
          transform: Transform::from_translation(HOLD_POS_FRUIT),
          ..default()
        }
      ));
    }
  }
}

fn pause_state(
  fruits: Query<&SpawnedFruit>,
) {
  // pause physics
  let mut len = 0;
  for _fruit in fruits.iter() {
    len += 1;
  }
  println!("len: {}", len);
}