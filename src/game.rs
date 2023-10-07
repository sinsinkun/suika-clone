// use std::time::Duration;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::util::{
  AppState,
  Score,
  SpawnedFruit,
  // CoolDown,
  // SCREEN_H,
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
  GRAVITY,
  RESTITUATION,
  MIN_SPEED,
  // CLICK_DELAY,
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
  // setup initial state
  // spawn cup base with collider
  let container_base = -0.75 * CONTAINER_H;
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
      mesh: meshes.add(shape::Circle::new(SUIKA[5].size + 1.0).into()).into(),
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
  spawn_fruits: Query<(&Transform, &Velocity), (With<SpawnedFruit>, Without<ActiveFruit>, Without<NextFruit>)>,
) {
  // quick exit
  if keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::Escape) {
    next_state.set(AppState::GameOver);
  }

  // find if fruit has exceeded limits
  let max_h = 0.25 * CONTAINER_H - CONTAINER_T;
  let max_x = 0.5 * CONTAINER_W + CONTAINER_T;
  for (fruit_t, fruit_v) in spawn_fruits.iter() {
    if fruit_t.translation.x > max_x {
      println!("Game Over: fruit has gone outside right boundary {}", fruit_t.translation.x);
      next_state.set(AppState::GameOver);
    }
    if fruit_t.translation.x < -max_x {
      println!("Game Over: fruit has gone outside left boundary {}", fruit_t.translation.x);
      next_state.set(AppState::GameOver);
    }
    
    let scalar_v = fruit_v.linvel.length();
    if scalar_v.abs() < MIN_SPEED && fruit_t.translation.y > max_h {
      println!("Game Over: fruit has reached max height");
      next_state.set(AppState::GameOver);
    }
  };
}

fn handle_active_fruit(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut active_fruit_q: Query<(Entity, &mut Transform, &ActiveFruit), With<ActiveFruit>>,
  next_fruit_q: Query<&NextFruit>,
  keys: Res<Input<KeyCode>>,
) {
  // spawn active fruit if not exist
  match active_fruit_q.get_single_mut() {
    Ok((entity, transform, active_fruit)) => {
      // let cd = &mut cool_down.into_inner().timer;
      // cd.tick(time.delta());
      // println!("timer: {:?}", cd);
      // spawn active fruit
      if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Return) {

        let cur_fruit = SUIKA[active_fruit.0 as usize];
        let cur_x = transform.into_inner().translation.x;
        // spawn collision fruit body
        commands.spawn((
          SpawnedFruit,
          Collider::ball(cur_fruit.size / 2.0),
          RigidBody::Dynamic,
          GravityScale(GRAVITY),
          Restitution::coefficient(RESTITUATION),
          Velocity {linvel: Vec2::new(0.0, 0.0), angvel: 0.4},
          ActiveEvents::COLLISION_EVENTS,
          MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(cur_fruit.size / 2.0).into()).into(),
            material: materials.add(ColorMaterial::from(cur_fruit.color)),
            transform: Transform::from_translation(Vec3::new(cur_x, CONTAINER_H / 2.0, 0.5)),
            ..default()
          },
        ));

        // despawn active fruit
        commands.entity(entity).despawn_recursive();

        // pick next fruit
        let num: i32 = match next_fruit_q.get_single() {
          Ok(next_fruit) => next_fruit.0,
          Err(_) => rand::thread_rng().gen_range(0..5)
        };
        let active_fruit = SUIKA[num as usize];

        // spawn new active fruit
        commands.spawn((
          ActiveFruit(num),
          SpawnedFruit,
          MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(active_fruit.size / 2.0).into()).into(),
            material: materials.add(ColorMaterial::from(active_fruit.color)),
            transform: Transform::from_translation(Vec3::new(cur_x, CONTAINER_H / 2.0, 0.5)),
            ..default()
          },
        )).with_children(|root| {
          // spawn preview bar
          root.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(1.5, 1.25 * CONTAINER_H)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0.0, -0.625 * CONTAINER_H, -1.0)),
            ..default()
          });
        });

        // start cooldown
        return;
      }
      
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
      let limit1 = CONTAINER_W / 2.0 - CONTAINER_P;
      let suika_num = active_fruit.0;
      let limit2 = (CONTAINER_W - CONTAINER_T - SUIKA[suika_num as usize].size) / 2.0;
      // set maximum travel distance
      let limit = if limit1 < limit2 {
        limit1
      } else {
        limit2
      };
      if new_x > -limit && new_x < limit {
        transform.into_inner().translation.x = new_x;
      } else if new_x >= limit {
        transform.into_inner().translation.x = limit;
      } else if new_x <= -limit {
        transform.into_inner().translation.x = -limit;
      }
    },
    Err(_) => {
      // pick next fruit
      let num: i32 = match next_fruit_q.get_single() {
        Ok(next_fruit) => next_fruit.0,
        Err(_) => rand::thread_rng().gen_range(0..4)
      };
      let active_fruit = SUIKA[num as usize];

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
          mesh: meshes.add(shape::Quad::new(Vec2::new(1.5, 1.25 * CONTAINER_H)).into()).into(),
          material: materials.add(ColorMaterial::from(Color::WHITE)),
          transform: Transform::from_translation(Vec3::new(0.0, -0.625 * CONTAINER_H, -1.0)),
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
  mut next_fruit_q: Query<Entity, With<NextFruit>>,
  keys: Res<Input<KeyCode>>,
) {
  // spawn active fruit if not exist
  match next_fruit_q.get_single_mut() {
    Ok(entity) => {
      if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::S) {
        // despawn NextFruit
        commands.entity(entity).despawn_recursive();
        // spawn new NextFruit
        // pick random fruit
        let num: i32 = rand::thread_rng().gen_range(0..5);
        let next_fruit = SUIKA[num as usize];

        // spawn new NextFruit
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
        return;
      }
    },
    Err(_) => {
      // pick random fruit
      let num: i32 = rand::thread_rng().gen_range(0..4);
      let next_fruit = SUIKA[num as usize];

      // spawn new NextFruit
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

fn pause_state() {
  // pause physics
}