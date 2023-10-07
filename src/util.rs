use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Fruit {
	pub size: f32,
	pub score: i32,
	pub color: Color,
}

impl Fruit {
	const fn new(size:f32, score:i32, color:Color) -> Self {
		Fruit { size, score, color }
	}
}

// ---- SCENES ----
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
	#[default]
	Menu,
	InGame,
	GameOver,
}

// ---- RESOURCES ----

// ---- COMPONENTS ----
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CoolDown {
  pub timer: Timer
}

#[derive(Component)]
pub struct Score(pub i32, pub i32);

#[derive(Component)]
pub struct SpawnedFruit;

// ---- CONSTANTS ----
use bevy::prelude::Color;

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
// pub const ZERO_POS: Vec3 = Vec3::new(-9999.0, -9999.0, 0.1);

// physics
pub const GRAVITY: f32 = 4.0;
pub const RESTITUATION: f32 = 0.01;
// pub const MASS: f32 = 5.0;
// pub const MAX_SPEED: f32 = 100.0;
// pub const MAX_Y_VELOCITY_BEFORE_CLAMP: f32 = 50.0;
// pub const MAX_X_VELOCITY_BEFORE_CLAMP: f32 = 50.0;
pub const MIN_SPEED: f32 = 3.0;

// colors
pub const BG_COLOR: Color = Color::rgb(0.7843, 0.6549, 0.3373);
pub const BG_NO_MOVE_COLOR: Color = Color::rgb(0.73, 0.6, 0.28);
pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const CONTAINER_COLOR: Color = Color::rgb(0.24, 0.42, 0.33);

// game objects
pub const CLICK_DELAY: f32 = 0.5;
pub const MOVE_SPEED: f32 = 2.8;
pub const SUIKA: [Fruit; 11] = [
  Fruit::new(31.2, 0, Color::rgb(0.3373, 0.5686, 0.7843)),
  Fruit::new(48.0, 1, Color::rgb(0.3804, 0.5373, 0.7922)),
  Fruit::new(64.8, 3, Color::rgb(0.4431, 0.5059, 0.7882)),
  Fruit::new(72.0, 6, Color::rgb(0.5137, 0.4667, 0.7686)),
  Fruit::new(92.4, 10, Color::rgb(0.5882, 0.4196, 0.7333)),
  Fruit::new(110.4, 15, Color::rgb(0.6549, 0.3686, 0.6824)),
  Fruit::new(116.4, 21, Color::rgb(0.7098, 0.3137, 0.6196)),
  Fruit::new(154.8, 28, Color::rgb(0.7569, 0.2549, 0.5373)),
  Fruit::new(184.8, 36, Color::rgb(0.7843, 0.1922, 0.4471)),
  Fruit::new(208.8, 45, Color::rgb(0.7922, 0.1412, 0.3490)),
  Fruit::new(244.8, 55, Color::rgb(0.7843, 0.1176, 0.2431))
];