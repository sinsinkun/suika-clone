// prevent console on release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_persistent::prelude::*;

#[cfg(debug_assertions)]
use bevy::input::touch::TouchPhase;

mod util;
use util::{AppState, Score, BG_COLOR, SCREEN_H, SCREEN_W, MainCamera, HighScore};

mod menu;
use menu::MenuPlugin;

mod game;
use game::InGamePlugin;

fn main() {

	// set persistent save location
	let mut persistent_path = "./save.bin";
	if cfg!(target_arch = "wasm32") {
		persistent_path = "local/save.bin";
	}

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
		.insert_resource(Score(0, 0))
		.insert_resource(Persistent::<HighScore>::builder()
			.name("high scores")
			.format(StorageFormat::Bincode)
			.path(persistent_path)
			.default(HighScore([0,0,0,0,0,0,0,0]))
			.build()
			.expect("Err: Could not load high scores")
		)
		.add_state::<AppState>()
		.add_systems(Startup, initialize)
		.add_systems(Update, zoom_camera)
		.add_systems(Update, mock_touch)
		.add_plugins(InGamePlugin)
		.add_plugins(MenuPlugin)
		.run();
}

fn initialize(mut commands: Commands) {
	// spawn camera
  commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn zoom_camera(
	windows: Query<&Window>,
	mut camera: Query<&mut Transform, With<MainCamera>>,
) {
	// note: only works if there's a single camera + single window
	for mut transform in camera.iter_mut() {
		let window = windows.single();

		// rotate camera 90 deg if window h > window w
		if window.height() > window.width() * 1.1 {
			let delta_x = SCREEN_H / window.width();
			let delta_y = SCREEN_W / window.height();
			let delta = if delta_y > delta_x {
				delta_y
			} else {
				delta_x
			};
			transform.scale = Vec3::new(delta, delta, 1.0);
			transform.rotation = Quat::from_rotation_z(1.5708);
		} else {
			let delta_x = SCREEN_W / window.width();
			let delta_y = SCREEN_H / window.height();
			let delta = if delta_y > delta_x {
				delta_y
			} else {
				delta_x
			};
			transform.scale = Vec3::new(delta, delta, 1.0);
			transform.rotation = Quat::from_rotation_z(0.0);
		}
	}
}

#[cfg(debug_assertions)]
pub fn mock_touch(
	mouse: Res<Input<MouseButton>>,
	windows: Query<&Window>,
	mut touch_events: EventWriter<TouchInput>,
) {
	let window = windows.single();
	let touch_phase = if mouse.just_pressed(MouseButton::Left) {
		Some(TouchPhase::Started)
	} else if mouse.just_released(MouseButton::Left) {
		Some(TouchPhase::Ended)
	} else if mouse.pressed(MouseButton::Left) {
		Some(TouchPhase::Moved)
	} else {
		None
	};
	if let (Some(phase), Some(cursor_pos)) = (touch_phase, window.cursor_position()) {
		touch_events.send(TouchInput {
			phase: phase,
			position: cursor_pos,
			force: None,
			id: 0,
		})
	}
}

#[cfg(not(debug_assertions))]
pub fn mock_touch() {}