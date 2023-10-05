// prevent console on release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod util;
use util::{AppState, BG_COLOR, SCREEN_H, SCREEN_W, MainCamera, Score};

mod menu;
use menu::MenuPlugin;

mod game;
use game::InGamePlugin;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Suika Clone".into(),
				resolution: (SCREEN_W, SCREEN_H).into(),
				..default()
			}),
			..default()
		}))
		.add_plugins((
			RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
			// RapierDebugRenderPlugin::default(),
		))
		.insert_resource(ClearColor(BG_COLOR))
		.add_state::<AppState>()
		.add_systems(Startup, initialize)
		.add_plugins(InGamePlugin)
		.add_plugins(MenuPlugin)
		.run();
}

pub fn initialize(mut commands: Commands) {
	// spawn camera
  commands.spawn((Camera2dBundle::default(), MainCamera));
	
	// initialize score
	commands.spawn(Score(0, 1000));

}