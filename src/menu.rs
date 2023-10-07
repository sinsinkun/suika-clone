use bevy::prelude::*;

use crate::util::{AppState, Fruit, TEXT_COLOR};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), setup_menu.after(in_state(AppState::Menu)))
      .add_systems(Update, on_loop.run_if(in_state(AppState::Menu)))
      .add_systems(OnExit(AppState::Menu), cleanup)
      .add_systems(OnEnter(AppState::GameOver), setup_game_over)
      .add_systems(Update, on_loop.run_if(in_state(AppState::GameOver)))
      .add_systems(OnExit(AppState::GameOver), cleanup);
  }
}

#[derive(Component)]
pub struct MenuItem;

fn setup_menu(mut commands: Commands) {

  // click anywhere text
  commands.spawn((
    MenuItem,
    Text2dBundle {
      text: Text::from_section(
        "click anywhere to begin",
        TextStyle {
          font_size: 30.0,
          color: TEXT_COLOR,
          ..default()
        },
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
      ..default()
    },
  ));
}

fn setup_game_over(mut commands: Commands) {

  // game over text
  commands.spawn((
    MenuItem,
    Text2dBundle {
      text: Text::from_section(
        "game over",
        TextStyle { 
          font_size: 30.0, 
          color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 40.0, 10.0)),
      ..default()
    },
  ));

  // score
  commands.spawn((
    MenuItem,
    Text2dBundle {
      text: Text::from_section(
        "Score: TODO",
        TextStyle {
          font_size: 30.0, 
        color: TEXT_COLOR,
          ..default()
        }
      ),
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
      ..default()
    }
  ));

  // high score
  commands.spawn((
    MenuItem,
    Text2dBundle {
      text: Text::from_section(
        "High score: TODO",
        TextStyle {
          font_size: 30.0, 
        color: TEXT_COLOR,
          ..default()
        }
      ).with_alignment(TextAlignment::Center),
      transform: Transform::from_translation(Vec3::new(0.0, -30.0, 10.0)),
      ..default()
    }
  ));
}

fn on_loop(
  mut next_state: ResMut<NextState<AppState>>,
  mouse_button_input: Res<Input<MouseButton>>,
  keys: Res<Input<KeyCode>>,
) {
  
  if keys.just_pressed(KeyCode::Space) || 
    keys.just_pressed(KeyCode::Return) || 
    mouse_button_input.just_pressed(MouseButton::Left) {
    next_state.set(AppState::InGame)
  }
}

fn cleanup(
  mut commands: Commands, 
  menu_items: Query<Entity, With<MenuItem>>,
  fruits: Query<Entity, With<Fruit>>,
) {
  for menu_item in menu_items.iter() {
    commands.entity(menu_item).despawn_recursive();
  }
  for fruit in fruits.iter() {
    commands.entity(fruit).despawn_recursive();
  }
}