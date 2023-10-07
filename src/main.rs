// prevent console on release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, window::WindowResized};
use bevy_rapier2d::prelude::*;

mod util;
use util::{AppState, Score, BG_COLOR, SCREEN_H, SCREEN_W, MainCamera};

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
		.insert_resource(Score(0, 1000))
		.add_state::<AppState>()
		.add_systems(Startup, initialize)
		.add_systems(Update, zoom_camera)
		.add_plugins(InGamePlugin)
		.add_plugins(MenuPlugin)
		.run();
}

fn initialize(mut commands: Commands) {
	// spawn camera
  commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn zoom_camera(
	mut resize_reader: EventReader<WindowResized>,
	mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
	// note: only works if there's a single camera + single window
	for e in resize_reader.iter() {
		// find largest change
		let delta_x = SCREEN_W / e.width;
		let delta_y = SCREEN_H / e.height;

		let delta = if delta_y > delta_x {
			delta_y
		} else {
			delta_x
		};

		for mut projection in camera.iter_mut() {
			projection.scale = delta;
		}
	}
}