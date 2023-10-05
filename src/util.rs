use bevy::prelude::*;

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
pub struct Sec(pub Timer);

// ---- COMPONENTS ----