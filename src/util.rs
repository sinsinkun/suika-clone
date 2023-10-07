use bevy::prelude::*;
// use bevy_persistent::prelude::*;

// ---- SCENES ----
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
	#[default]
	Menu,
	InGame,
	GameOver,
}

// ---- RESOURCES ----
#[derive(Resource)]
pub struct Score (pub i32, pub i32);

// ---- COMPONENTS ----
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CoolDown {
  pub timer: Timer
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Fruit {
  pub id: i32,
	pub size: f32,
	pub score: i32,
	pub color: Color,
}

impl Fruit {
	const fn new(id:i32, size:f32, score:i32, color:Color) -> Self {
		Fruit { id, size, score, color }
	}
}

// ---- CONSTANTS ----
// sizing
pub const SCREEN_W: f32 = 1200.0;
pub const SCREEN_H: f32 = 800.0;
pub const CONTAINER_W: f32 = 400.0;
pub const CONTAINER_H: f32 = 500.0;
pub const CONTAINER_T: f32 = 12.0;
pub const CONTAINER_P: f32 = 38.0;

// positions
pub const HOLD_POS: Vec3 = Vec3::new(350.0, 200.0, 0.0);
pub const HOLD_POS_FRUIT: Vec3 = Vec3::new(350.0, 200.0, 0.5);

// physics
pub const GRAVITY: f32 = 8.0;
pub const DAMPENING: f32 = 0.6;
pub const RESTITUATION: f32 = 0.005;
pub const FRICTION: f32 = 0.4;
pub const MIN_SPEED: f32 = 3.0;

// colors
pub const BG_COLOR: Color = Color::rgb(0.7843, 0.6549, 0.3373);
pub const BG_NO_MOVE_COLOR: Color = Color::rgb(0.73, 0.6, 0.28);
pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const CONTAINER_COLOR: Color = Color::rgb(0.24, 0.42, 0.33);

// game objects
pub const CLICK_DELAY: f32 = 0.4;
pub const MOVE_SPEED: f32 = 2.8;
pub const SUIKA: [Fruit; 11] = [
  Fruit::new(0, 31.2, 1, Color::rgb(0.3373, 0.5686, 0.7843)),
  Fruit::new(1, 48.0, 3, Color::rgb(0.3804, 0.5373, 0.7922)),
  Fruit::new(2, 64.8, 6, Color::rgb(0.4431, 0.5059, 0.7882)),
  Fruit::new(3, 72.0, 10, Color::rgb(0.5137, 0.4667, 0.7686)),
  Fruit::new(4, 92.4, 15, Color::rgb(0.5882, 0.4196, 0.7333)),
  Fruit::new(5, 110.4, 21, Color::rgb(0.6549, 0.3686, 0.6824)),
  Fruit::new(6, 116.4, 28, Color::rgb(0.7098, 0.3137, 0.6196)),
  Fruit::new(7, 154.8, 36, Color::rgb(0.7569, 0.2549, 0.5373)),
  Fruit::new(8, 184.8, 45, Color::rgb(0.7843, 0.1922, 0.4471)),
  Fruit::new(9, 208.8, 55, Color::rgb(0.7922, 0.1412, 0.3490)),
  Fruit::new(10, 244.8, 70, Color::rgb(0.7843, 0.1176, 0.2431)),
];