use std::time::Duration;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::util::{
  AppState,
  Score,
  Fruit,
  CoolDown,
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
  CLICK_DELAY,
  TEXT_COLOR, 
  FRICTION,
  DAMPENING,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (spawn_cup, spawn_permanent_ui))
      .add_systems(OnEnter(AppState::InGame), spawn_temp_ui)
      .add_systems(Update, (
          end_game,
          handle_inputs,
          handle_active_fruit,
          handle_next_fruit,
          handle_merging,
          update_score,
        ).run_if(in_state(AppState::InGame)))
      .add_systems(OnExit(AppState::InGame), pause_state);
  }
}

// -- COMPONENTS --
#[derive(Component)]
struct UIComponent;

#[derive(Component)]
struct PermUIComponent;

#[derive(Component)]
struct UIScore;

#[derive(Component)]
struct Cup;

#[derive(Component)]
struct ActiveFruit(i32);

#[derive(Component)]
struct NextFruit(i32);

#[derive(Component)]
struct PreviewBar;

#[derive(Component)]
struct Controls {
  move_dir: f32,
  enter: bool,
  end_game: bool,
}

// -- SYSTEMS --
fn spawn_cup(mut commands: Commands) {
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
        1.0,
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
        1.0,
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
        -2.0,
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
        -2.0,
      ),
      ..default()
    },
  ));
}

fn spawn_permanent_ui(
  mut commands: Commands, 
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // render hold area
  commands.spawn((
    PermUIComponent,
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(SUIKA[4].size).into()).into(),
      material: materials.add(ColorMaterial::from(BG_NO_MOVE_COLOR)),
      transform: Transform::from_translation(HOLD_POS),
      ..default()
    }
  ));

  // render score area
  let x = -HOLD_POS.x;
  let y = HOLD_POS.y;
  commands.spawn((
    PermUIComponent,
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(SUIKA[4].size).into()).into(),
      material: materials.add(ColorMaterial::from(BG_NO_MOVE_COLOR)),
      transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
      ..default()
    }
  ));
}

fn spawn_temp_ui(
  mut commands: Commands,
  mut score: ResMut<Score>,
) {
  // insantiate controls
  commands.spawn((
    Controls{ move_dir:0.0, enter:false, end_game:false },
    CoolDown{ timer:Timer::new(Duration::from_secs_f32(CLICK_DELAY), TimerMode::Once) }
  ));

  // spawn score area
  let x = -HOLD_POS.x;
  let y = HOLD_POS.y;
  score.0 = 0;
  let hs = "High Score: ".to_owned() + &score.1.to_string();

  // instantiate score
  commands.spawn((
    UIComponent,
    SceneBundle {
      transform: Transform::from_translation(Vec3::new(x, y, 10.0)),
      ..default()
    }
  )).with_children(|root| {
    // score render
    root.spawn((Text2dBundle {
      text: Text::from_section(
        "Score: 0",
        TextStyle {
          font_size: 20.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
      ..default()
    }, UIScore));
    // high score render
    root.spawn(Text2dBundle {
      text: Text::from_section(
        hs,
        TextStyle {
          font_size: 20.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)),
      ..default()
    });

  });
}

fn end_game(
  mut next_state: ResMut<NextState<AppState>>,
  controls: Query<&Controls>,
  spawn_fruits: Query<(&Transform, &Velocity), With<Fruit>>,
) {
  let input = controls.single();
  // quick exit
  if input.end_game {
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

fn handle_inputs(
  mut controls: Query<(&mut Controls, &mut CoolDown)>,
  keys: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  match controls.get_single_mut() {
    Ok((mut controls, mut cooldown)) => {
      cooldown.timer.tick(time.delta());
      if keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::Escape) {
        controls.end_game = true;
      }
      if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Return) {
        if cooldown.timer.finished() {
          controls.enter = true;
          cooldown.timer.reset();
        } 
      } else {
        controls.enter = false;
      }
      let mut move_dir = 0.0;
      if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
        move_dir -= 1.0;
      }
      if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
        move_dir += 1.0;
      }
      controls.move_dir = move_dir;
    },
    Err(_) => {
      println!("Couldn't find controls instance");
    }
  }
  
}

fn handle_active_fruit(
  mut commands: Commands,
  controls: Query<(&Controls, &CoolDown)>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut active_fruit_q: Query<(Entity, &mut Transform, &ActiveFruit), With<ActiveFruit>>,
  next_fruit_q: Query<&NextFruit>,
) {
  let (input, _cooldown) = controls.single();

  // get active fruit
  match active_fruit_q.get_single_mut() {
    Ok((entity, transform, active_fruit)) => {
      // spawn active fruit
      if input.enter {
        let cur_fruit = SUIKA[active_fruit.0 as usize];
        let cur_x = transform.into_inner().translation.x;
        let cur_y = CONTAINER_H / 2.0;
        let pos = Vec3::new(cur_x, cur_y, 2.0);
        
        // spawn collision fruit body
        spawn_collider_fruit(&mut commands, &mut meshes,  &mut materials, cur_fruit, pos);

        // despawn active fruit
        commands.entity(entity).despawn_recursive();

        // pick next fruit
        let num: i32 = match next_fruit_q.get_single() {
          Ok(next_fruit) => next_fruit.0,
          Err(_) => rand::thread_rng().gen_range(0..4)
        };
        let active_fruit = SUIKA[num as usize];
        let pos = Vec3::new(cur_x, CONTAINER_H / 2.0, 1.0);
        spawn_active_fruit(&mut commands, &mut meshes, &mut materials, active_fruit, pos);

        // prevent further active control
        return;
      }
      
      // calculations for updating active fruit
      let new_x = transform.clone().translation.x + MOVE_SPEED * input.move_dir;
      let limit1 = CONTAINER_W / 2.0 - CONTAINER_P;
      let suika_num = active_fruit.0;
      let limit2 = (CONTAINER_W - CONTAINER_T - SUIKA[suika_num as usize].size) / 2.0;
      // set maximum travel distance
      let limit = if limit1 < limit2 {
        limit1
      } else {
        limit2
      };
      // update active fruit render
      if new_x > -limit && new_x < limit {
        transform.into_inner().translation.x = new_x;
      } else if new_x >= limit {
        transform.into_inner().translation.x = limit;
      } else if new_x <= -limit {
        transform.into_inner().translation.x = -limit;
      }
    },
    Err(e) => {
      println!("Active Fruit Err: {:?}", e);
      // pick new fruit
      let num: i32 = match next_fruit_q.get_single() {
        Ok(next_fruit) => next_fruit.0,
        Err(_) => rand::thread_rng().gen_range(0..4)
      };
      let active_fruit = SUIKA[num as usize];
      let pos = Vec3::new(0.0, CONTAINER_H / 2.0, 1.0);
      spawn_active_fruit(&mut commands, &mut meshes, &mut materials, active_fruit, pos);

    }
  }
}

fn handle_next_fruit(
  mut commands: Commands,
  controls: Query<&Controls>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut next_fruit_q: Query<Entity, With<NextFruit>>,
) {
  let input = controls.single();
  // spawn next fruit if not exist
  match next_fruit_q.get_single_mut() {
    Ok(entity) => {
      if input.enter {
        // despawn NextFruit
        commands.entity(entity).despawn_recursive();
        // spawn new NextFruit
        // pick random fruit
        let num: i32 = rand::thread_rng().gen_range(0..4);
        let next_fruit = SUIKA[num as usize];
        spawn_next_fruit(&mut commands, &mut meshes, &mut materials, next_fruit);
      }
    },
    Err(_) => {
      // pick random fruit
      let num: i32 = rand::thread_rng().gen_range(0..4);
      let next_fruit = SUIKA[num as usize];
      spawn_next_fruit(&mut commands, &mut meshes, &mut materials, next_fruit);
    }
  }
}

fn handle_merging(
  mut commands: Commands,
  mut collisions: EventReader<CollisionEvent>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  fruits: Query<(Entity, &Fruit, &Transform)>,
  mut score: ResMut<Score>,
) {
  for collision in collisions.iter() {
    if let CollisionEvent::Started(collider_a, collider_b, _) = collision {
      // get fruits from collision, if it was a collision between fruits
      if let Ok([fruit_a, fruit_b]) = fruits.get_many([*collider_a, *collider_b]) {
        if fruit_a.1.size == fruit_b.1.size && fruit_a.1.id < 10 {
          // calculate midpoint between 2 fruits
          let new_translation = Vec3::new(
            (fruit_a.2.translation.x + fruit_b.2.translation.x) / 2.0,
            (fruit_a.2.translation.y + fruit_b.2.translation.y) / 2.0,
            2.0
          );
          let new_fruit = SUIKA[(fruit_a.1.id + 1) as usize];
          // remove collided fruits
          commands.entity(fruit_a.0).despawn_recursive();
          commands.entity(fruit_b.0).despawn_recursive();
          // spawn new fruit from SUIKA + 1
          spawn_collider_fruit(&mut commands,  &mut meshes, &mut materials, new_fruit, new_translation);
          // add points
          score.0 += fruit_a.1.score;
          // exit for loop - only calculate one successful merge per frame
          break;
        }
        
      }
    }
  }
}

fn update_score(
  mut score_q: Query<&mut Text, With<UIScore>>,
  score: ResMut<Score>,
) {
  if let Ok(score_t) = score_q.get_single_mut() {
    let text = score_t.into_inner();
    text.sections[0].value = "Score: ".to_owned() + &score.0.to_string();
  }
}

fn pause_state(
  mut commands: Commands,
  controls: Query<Entity, With<Controls>>,
  active_fruit: Query<Entity, With<ActiveFruit>>,
  next_fruit: Query<Entity, With<NextFruit>>,
  ui_elements: Query<Entity, With<UIComponent>>,
  mut score: ResMut<Score>,
) {
  // destroy components that should only have 1 existence
  commands.entity(controls.single()).despawn_recursive();
  commands.entity(active_fruit.single()).despawn_recursive();
  commands.entity(next_fruit.single()).despawn_recursive();

  // destroy ui elements only shown during gameplay
  for e in ui_elements.iter() {
    commands.entity(e).despawn_recursive();
  }

  // calculate high score:
  if score.0 > score.1 {
    score.1 = score.0;
  }
  // pause physics
}

// --- HELPER FUNCTIONS ---
fn spawn_active_fruit(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  fruit: Fruit,
  position: Vec3,
) {
  commands.spawn((
    ActiveFruit(fruit.id),
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(fruit.size / 2.0).into()).into(),
      material: materials.add(ColorMaterial::from(fruit.color)),
      transform: Transform::from_translation(position),
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
    // spawn text
    root.spawn(Text2dBundle {
      text: Text::from_section(
        fruit.id.to_string(),
        TextStyle {
          font_size: 20.0, 
        color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
      ..default()
    });
  });
}

fn spawn_next_fruit(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  fruit: Fruit,
) {
  commands.spawn((
    NextFruit(fruit.id),
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(fruit.size / 2.0).into()).into(),
      material: materials.add(ColorMaterial::from(fruit.color)),
      transform: Transform::from_translation(HOLD_POS_FRUIT),
      ..default()
    }
  )).with_children(|root| {
    root.spawn(Text2dBundle {
      text: Text::from_section(
        fruit.id.to_string(),
        TextStyle {
          font_size: 20.0, 
        color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
      ..default()
    });
  });
}

fn spawn_collider_fruit(
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  cur_fruit: Fruit,
  position: Vec3,
) {
  commands.spawn((
    cur_fruit,
    Collider::ball(cur_fruit.size / 2.0),
    ColliderMassProperties::Density(cur_fruit.size.log2()),
    Friction { coefficient: FRICTION, combine_rule: CoefficientCombineRule::Max },
    RigidBody::Dynamic,
    GravityScale(GRAVITY),
    Damping { linear_damping: DAMPENING, angular_damping: 1.0 },
    Restitution::coefficient(RESTITUATION),
    Velocity {linvel: Vec2::new(0.0, 0.0), angvel: 0.4},
    ActiveEvents::COLLISION_EVENTS,
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(cur_fruit.size / 2.0).into()).into(),
      material: materials.add(ColorMaterial::from(cur_fruit.color)),
      transform: Transform::from_translation(position),
      ..default()
    },
  )).with_children(|root| {
    root.spawn(Text2dBundle {
      text: Text::from_section(
        cur_fruit.id.to_string(),
        TextStyle {
          font_size: 20.0, 
        color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
      ..default()
    });
  });
}