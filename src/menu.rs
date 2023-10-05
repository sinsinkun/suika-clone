use bevy::prelude::*;

use crate::util::AppState;
use crate::constants::TEXT_COLOR;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_menu)
      .add_systems(Update, on_loop.run_if(in_state(AppState::Menu)))
      .add_systems(OnEnter(AppState::GameOver), setup_game_over)
      .add_systems(Update, on_loop.run_if(in_state(AppState::GameOver)))
      .add_systems(OnExit(AppState::GameOver), cleanup);
  }
}

#[derive(Component)]
pub struct MenuItem;

fn setup_menu(mut commands: Commands) {
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
      )
      .with_alignment(TextAlignment::Center),
      ..default()
    },
  ));
}

fn setup_game_over(mut commands: Commands) {
  commands.spawn((
    MenuItem,
    Text2dBundle {
      text: Text::from_section(
        "game over",
        TextStyle {
          font_size: 30.0,
          color: TEXT_COLOR,
          ..default()
        },
      )
      .with_alignment(TextAlignment::Center),
      ..default()
    },
  ));
}

fn on_loop(
  mut next_state: ResMut<NextState<AppState>>,
  state: Res<State<AppState>>, 
  mouse_button_input: Res<Input<MouseButton>>,
) {
  println!("On loop in menu {:?}", state);

  if mouse_button_input.just_pressed(MouseButton::Left) {
    if state.get() == &AppState::Menu {
      next_state.set(AppState::GameOver)
    }
    if state.get() == &AppState::GameOver {
      next_state.set(AppState::Menu)
    }
  }
}

fn cleanup() {
  println!("Clean up");
}