use std::time::Duration;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use bevy_persistent::prelude::Persistent;
use rand::Rng;

use crate::util::{
  AppState,
  Score,
  Fruit,
  CoolDown,
  HighScore,
  SCREEN_W,
  SCREEN_H,
  CONTAINER_W,
  CONTAINER_H,
  CONTAINER_T,
  CONTAINER_P,
  CONTAINER_COLOR,
  CUP_BG_COLOR,
  OVERLAY_COLOR,
  MAX_H_COLOR,
  SUIKA,
  HOLD_POS,
  HOLD_POS_FRUIT,
  LEGEND_POS,
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
    app.insert_resource(Positions {
        cup_base_y: -0.5 * CONTAINER_H - CONTAINER_P,
        cup_max_y: 0.5 * CONTAINER_H - CONTAINER_P,
        cup_left_x: -CONTAINER_W / 2.0,
        cup_right_x: CONTAINER_W / 2.0,
      })
      .add_systems(Startup, (spawn_cup, spawn_permanent_ui))
      .add_systems(OnEnter(AppState::InGame), reset_game_state)
      .add_systems(Update, (
          end_game,
          handle_inputs,
          handle_active_fruit.before(handle_next_fruit),
          handle_next_fruit,
          handle_merging,
          update_score,
        ).run_if(in_state(AppState::InGame)))
      .add_systems(OnExit(AppState::InGame), pause_state);
  }
}

// -- RESOURCES --
#[derive(Resource)]
struct Positions {
  cup_base_y: f32,
  cup_max_y: f32,
  cup_left_x: f32,
  cup_right_x: f32,
}

// -- COMPONENTS --
#[derive(Component)]
struct UIComponent;

#[derive(Component)]
struct PermUIComponent;

#[derive(Component)]
struct UIScore;

#[derive(Component)]
struct UIHighScore;

#[derive(Component)]
struct UIHighScoreList(usize);

#[derive(Component)]
struct Cup;

#[derive(Component)]
struct ActiveFruit(i32);

#[derive(Component)]
struct NextFruit(i32);

#[derive(Component)]
struct PreviewBar;

#[derive(Component)]
struct Timeout;

#[derive(Component, Debug)]
struct Controls {
  move_dir: f32,
  drop_lock: bool,
  drop: bool,
  end_game: bool,
}

// -- SYSTEMS --
fn spawn_cup(mut commands: Commands, positions: Res<Positions>) {
  let container_base = positions.cup_base_y - 0.5 * CONTAINER_T;
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
  let wall_base = container_base + 0.5 * CONTAINER_H;
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
        positions.cup_left_x - 0.5 * CONTAINER_T,
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
        positions.cup_right_x + 0.5 * CONTAINER_T,
        wall_base,
        1.0,
      ),
      ..default()
    },
  ));

  // spawn background
  let bg_x = positions.cup_right_x + positions.cup_left_x;
  let bg_y = positions.cup_max_y + positions.cup_base_y + CONTAINER_T * 2.0;
  commands.spawn((
    Cup,
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(CONTAINER_W, CONTAINER_H)),
        color: CUP_BG_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(bg_x, bg_y, -3.0),
      ..default()
    },
  ));

  // render unmovable zone left
  commands.spawn((
    Cup,
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(CONTAINER_P, wall_h)),
        color: OVERLAY_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(
        positions.cup_left_x + 0.5 * CONTAINER_P,
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
        color: OVERLAY_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(
        positions.cup_right_x - 0.5 * CONTAINER_P,
        wall_base,
        -2.0,
      ),
      ..default()
    },
  ));

  // render max height line
  commands.spawn((
    Cup,
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(CONTAINER_W + CONTAINER_T * 2.0, 1.5)),
        color: MAX_H_COLOR,
        ..default()
      },
      transform: Transform::from_xyz(0.0, positions.cup_max_y + 0.75, -3.0),
      ..default()
    },
  ));
}

fn spawn_permanent_ui(
  mut commands: Commands, 
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
  score: Res<Score>,
  highscore: Res<Persistent<HighScore>>,
) {
  // render hold area
  commands.spawn((
    PermUIComponent,
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(SUIKA[4].size).into()).into(),
      material: materials.add(ColorMaterial::from(OVERLAY_COLOR)),
      transform: Transform::from_translation(HOLD_POS),
      ..default()
    }
  )).with_children(|root| {
    // spawn text
    root.spawn(Text2dBundle {
      text: Text::from_section(
        "Next",
        TextStyle {
          font_size: 30.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, SUIKA[4].size, 0.0)),
      ..default()
    });
  });

  // render score area
  commands.spawn((
    PermUIComponent,
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(SUIKA[5].size).into()).into(),
      material: materials.add(ColorMaterial::from(OVERLAY_COLOR)),
      transform: Transform::from_translation(Vec3::new(-HOLD_POS.x, HOLD_POS.y, 0.0)),
      ..default()
    }
  )).with_children(|root| {
    // score text
    root.spawn(Text2dBundle {
      text: Text::from_section(
        "Score",
        TextStyle {
          font_size: 30.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, SUIKA[5].size, 0.0)),
      ..default()
    });
    // score render
    root.spawn((Text2dBundle {
      text: Text::from_section(
        "0",
        TextStyle {
          font_size: 40.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0)),
      ..default()
    }, UIScore));
    // high score text
    root.spawn(Text2dBundle {
      text: Text::from_section(
        "Best Score",
        TextStyle {
          font_size: 22.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)),
      ..default()
    });
    // high score render
    root.spawn((Text2dBundle {
      text: Text::from_section(
        score.1.to_string(),
        TextStyle {
          font_size: 30.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, -35.0, 0.0)),
      ..default()
    }, UIHighScore));
  });

  // render highscore area
  commands.spawn((
    PermUIComponent,
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Quad::new(Vec2::new(280.0, 320.0)).into()).into(),
      material: materials.add(ColorMaterial::from(OVERLAY_COLOR)),
      transform: Transform::from_translation(Vec3::new(-HOLD_POS.x, LEGEND_POS.y - 30.0, 0.0)),
      ..default()
    }
  )).with_children(|root| {
    // render title
    root.spawn(Text2dBundle {
      text: Text::from_section(
        "High Scores:",
        TextStyle {
          font_size: 30.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 140.0, 0.0)),
      ..default()
    });
    // render updated high scores
    for (i, hscore) in highscore.0.iter().enumerate() {
      let y = 80.0 - i as f32 * 30.0;
      root.spawn((Text2dBundle {
        text: Text::from_section(
          hscore.to_string(),
          TextStyle {
            font_size: 30.0,
            color: TEXT_COLOR,
            ..default()
          }
        ),
        transform: Transform::from_translation(Vec3::new(0.0, y, 10.0)),
        ..default()
      }, UIHighScoreList(i)));
    };
  });

  // render legend
  commands.spawn((
    PermUIComponent,
    SpriteBundle {
      texture: asset_server.load("suika_clone_legend.png"),
      transform: Transform {
        translation: LEGEND_POS,
        scale: Vec3::new(0.7, 0.7, 1.0), 
        ..default()
      },
      ..default()
    },
  ));

  // render controls explanation
  let controls_x = SCREEN_W / 4.0;
  let controls_y = 15.0 - SCREEN_H / 2.0;
  commands.spawn((
    PermUIComponent,
    Text2dBundle {
      text: Text::from_section(
        "Arrow keys: move | Space: drop | Esc: quit", 
        TextStyle {
          font_size: 18.0,
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(controls_x, controls_y, 10.0)),
      ..default()
    }
  ));
}

fn reset_game_state(
  mut commands: Commands,
  mut score: ResMut<Score>,
  mut rapier_config: ResMut<RapierConfiguration>,
  mut highscore_q: Query<&mut Text, With<UIHighScore>>,
) {
  // insantiate controls
  commands.spawn((
    Controls{ move_dir:0.0, drop_lock:false, drop:false, end_game:false },
    CoolDown{ timer:Timer::new(Duration::from_secs_f32(CLICK_DELAY), TimerMode::Once) }
  ));

  // reset score
  score.0 = 0;
  // update highscore
  if let Ok(text) = highscore_q.get_single_mut() {
    text.into_inner().sections[0].value = score.1.to_string();
  }

  // restart physics simulation
  rapier_config.physics_pipeline_active = true;

}

fn end_game(
  mut commands: Commands,
  positions: Res<Positions>,
  mut next_state: ResMut<NextState<AppState>>,
  controls: Query<&Controls>,
  spawned_fruits: Query<(&Transform, &Velocity, &Fruit)>,
  mut time_out: Query<(Entity, &mut CoolDown), With<Timeout>>,
  time: Res<Time>,
) {
  let input = controls.single();
  // quick exit
  if input.end_game {
    next_state.set(AppState::GameOver);
  }

  // tick timer
  if let Ok((e, cd)) = time_out.get_single_mut() {
    let timer = &mut cd.into_inner().timer;
    timer.tick(time.delta());

    // delete timer if time is over
    if timer.finished() {
      commands.entity(e).despawn_recursive();
    }
  }

  // find if fruit has exceeded limits
  let max_h = positions.cup_max_y;
  let max_x = positions.cup_right_x + CONTAINER_T;
  for (fruit_t, fruit_v, fruit) in spawned_fruits.iter() {
    if fruit_t.translation.x > max_x {
      println!("Game Over: fruit has gone outside right boundary {}", fruit_t.translation.x);
      next_state.set(AppState::GameOver);
    }
    if fruit_t.translation.x < -max_x {
      println!("Game Over: fruit has gone outside left boundary {}", fruit_t.translation.x);
      next_state.set(AppState::GameOver);
    }
    
    let scalar_v = fruit_v.linvel.length();
    if scalar_v.abs() < MIN_SPEED && fruit_t.translation.y > max_h - (0.4 * fruit.size) {
      // get timeout timer
      match time_out.get_single() {
        Ok((_, cooldown)) => {
          if cooldown.timer.finished() {
            println!("Game Over: fruit has reached max height");
            next_state.set(AppState::GameOver);
          } else {
            println!("Game Over imminent: fruit is past max height");
          }
        },
        Err(_) => {
          // spawn timeout timer
          commands.spawn((
            CoolDown {timer:Timer::from_seconds(0.5, TimerMode::Once)},
            Timeout
          ));
        }
      }
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
      if cooldown.timer.just_finished() {
        controls.drop_lock = false;
      }
      if keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::Escape) {
        controls.end_game = true;
      }
      if !controls.drop_lock && (keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Return)) {
        controls.drop = true;
        controls.drop_lock = true;
        cooldown.timer.reset();
      } else {
        controls.drop = false;
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
  positions: Res<Positions>,
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
      if input.drop {
        let cur_fruit = SUIKA[active_fruit.0 as usize];
        let cur_translation = transform.into_inner().translation;
        let cur_z = rand::thread_rng().gen_range(2.0..5.0);
        let pos = Vec3::new(cur_translation.x, cur_translation.y, cur_z);
        
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
        spawn_active_fruit(&mut commands, &positions, &mut meshes, &mut materials, active_fruit, cur_translation.x);

        // prevent further active control
        return;
      }
      
      // calculations for updating active fruit
      let new_x = transform.clone().translation.x + MOVE_SPEED * input.move_dir;
      let suika_num = active_fruit.0;
      let limit = positions.cup_right_x - SUIKA[suika_num as usize].size / 2.0;
      // update active fruit render
      if new_x > -limit && new_x < limit {
        transform.into_inner().translation.x = new_x;
      } else if new_x >= limit {
        transform.into_inner().translation.x = limit;
      } else if new_x <= -limit {
        transform.into_inner().translation.x = -limit;
      }
    },
    Err(_e) => {
      // pick new fruit
      let num: i32 = match next_fruit_q.get_single() {
        Ok(next_fruit) => next_fruit.0,
        Err(_) => rand::thread_rng().gen_range(0..3)
      };
      let active_fruit = SUIKA[num as usize];
      spawn_active_fruit(&mut commands, &positions, &mut meshes, &mut materials, active_fruit, 0.0);

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
      if input.drop {
        // despawn NextFruit
        commands.entity(entity).despawn_recursive();
        // spawn new NextFruit
        // pick random fruit
        let num: i32 = rand::thread_rng().gen_range(0..5);
        let next_fruit = SUIKA[num as usize];
        spawn_next_fruit(&mut commands, &mut meshes, &mut materials, next_fruit);
      }
    },
    Err(_) => {
      // pick random fruit
      let num: i32 = rand::thread_rng().gen_range(0..5);
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
            rand::thread_rng().gen_range(2.0..5.0)
          );
          let new_fruit = SUIKA[(fruit_a.1.id + 1) as usize];
          // remove collided fruits
          commands.entity(fruit_a.0).despawn_recursive();
          commands.entity(fruit_b.0).despawn_recursive();
          // spawn new fruit from SUIKA + 1
          spawn_collider_fruit(&mut commands,  &mut meshes, &mut materials, new_fruit, new_translation);
          // add points
          score.0 += new_fruit.score;
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
    text.sections[0].value = score.0.to_string();
  }
}

fn pause_state(
  mut commands: Commands,
  controls: Query<Entity, With<Controls>>,
  active_fruit: Query<Entity, With<ActiveFruit>>,
  next_fruit: Query<Entity, With<NextFruit>>,
  ui_elements: Query<Entity, With<UIComponent>>,
  mut score: ResMut<Score>,
  mut highscore: ResMut<Persistent<HighScore>>,
  mut rapier_config: ResMut<RapierConfiguration>,
  mut highscore_list: Query<(&mut Text, &UIHighScoreList)>,
) {
  // destroy components that should only have 1 existence
  commands.entity(controls.single()).despawn_recursive();
  commands.entity(active_fruit.single()).despawn_recursive();
  commands.entity(next_fruit.single()).despawn_recursive();

  // destroy ui elements only shown during gameplay
  for e in ui_elements.iter() {
    commands.entity(e).despawn_recursive();
  }

  // TODO: add remaining fruits to score

  // calculate best score:
  if score.0 > score.1 {
    score.1 = score.0;
  }
  // update persistent high score
  let mut temp_score = score.0;
  for hscore in highscore.0.iter_mut() {
    if temp_score > *hscore {
      let temp = temp_score;
      temp_score = *hscore;
      *hscore = temp;
    }
  }
  highscore.persist().ok();

  // update highscore list
  for (text, hs_list) in highscore_list.iter_mut() {
    text.into_inner().sections[0].value = highscore.0[hs_list.0].to_string();
  }

  // pause physics
  rapier_config.physics_pipeline_active = false;
}

// --- HELPER FUNCTIONS ---
fn spawn_active_fruit(
  commands: &mut Commands,
  cup_pos: &Positions,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<ColorMaterial>>,
  fruit: Fruit,
  x_pos: f32,
) {
  let active_fruit_y = cup_pos.cup_max_y + SUIKA[5].size / 2.0;
  let preview_bar_y = cup_pos.cup_base_y;
  let preview_bar_h = active_fruit_y - cup_pos.cup_base_y;

  commands.spawn((
    ActiveFruit(fruit.id),
    MaterialMesh2dBundle {
      mesh: meshes.add(shape::Circle::new(fruit.size / 2.0).into()).into(),
      material: materials.add(ColorMaterial::from(fruit.color)),
      transform: Transform::from_translation(Vec3::new(x_pos, active_fruit_y, 1.0)),
      ..default()
    },
  )).with_children(|root| {
    // spawn preview bar
    root.spawn(MaterialMesh2dBundle {
      mesh: meshes.add(shape::Quad::new(Vec2::new(1.5, preview_bar_h)).into()).into(),
      material: materials.add(ColorMaterial::from(Color::WHITE)),
      transform: Transform::from_translation(Vec3::new(0.0, preview_bar_y, -1.0)),
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

  let angular_vel = (position.z - 3.5) * 0.2;

  commands.spawn((
    cur_fruit,
    Collider::ball(cur_fruit.size / 2.0),
    ColliderMassProperties::Density((cur_fruit.size + 10.0).log10()),
    Friction { coefficient: FRICTION, combine_rule: CoefficientCombineRule::Max },
    RigidBody::Dynamic,
    GravityScale(GRAVITY),
    Damping { linear_damping: DAMPENING, angular_damping: 0.0 },
    Restitution::coefficient(RESTITUATION),
    Velocity {linvel: Vec2::new(0.0, 0.0), angvel: angular_vel},
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
