use bevy::prelude::*;

use crate::util::AppState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_game)
      .add_systems(Update, on_loop.run_if(in_state(AppState::InGame)))
      .add_systems(OnExit(AppState::InGame), pause_state);
  }
}

fn setup_game() {
  // initialize physics
  // setup initial state
}

fn on_loop() {
  // read inputs
  // calculate physics
  // calculate score
}

fn pause_state() {
  // pause physics
}