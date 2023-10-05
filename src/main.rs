// prevent console on release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod constants;
use constants::{BG_COLOR, SCREEN_H, SCREEN_W};

mod util;
use util::AppState;

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
				fit_canvas_to_parent: true,
				prevent_default_event_handling: false,
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
		.add_plugins(MenuPlugin)
		.add_plugins(InGamePlugin)
		.run();
}
