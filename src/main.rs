#![allow(clippy::type_complexity)]

mod animation;
mod camera;
mod character;
mod game;
mod lighting;
mod rendering;
mod text;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use game::{play_audio_system, ContourPlugins, GameState};
use rendering::RenderingPlugin;

fn main() {
    App::new()
        // Setup stuff
        .add_plugin(RenderingPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(EguiPlugin)
        // Physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // Game specific
        .add_state(GameState::Prelude)
        .add_state_to_stage(CoreStage::PostUpdate, GameState::Prelude)
        .add_plugins(ContourPlugins)
        .add_startup_system(play_audio_system)
        .run();
}
