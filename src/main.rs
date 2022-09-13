#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod ai;
pub mod assets;
mod audio;
mod config;
mod consts;
mod game;
mod pawn;
mod rendering;
mod ui;

use audio::ContourAudioPlugins;
#[cfg(debug_assertions)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;
#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;
#[cfg(debug_assertions)]
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use config::ConfigPlugin;
use game::GameState;
use pawn::ContourPawnPlugins;
use rendering::ContourRenderingPlugins;
use ui::ContourUiPlugins;

fn main() {
    let mut app = App::new();
    // Setup stuff

    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Contour".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            transparent: true,
            position: WindowPosition::At(Vec2 { x: 1720.0, y: 0.0 }),
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(EguiPlugin)
        // Game specific
        .add_state(GameState::Prelude)
        .add_state_to_stage(CoreStage::PostUpdate, GameState::Prelude)
        .add_plugin(ConfigPlugin)
        .add_plugins(ContourRenderingPlugins)
        .add_plugins(ContourPawnPlugins)
        .add_plugins(ContourAudioPlugins)
        .add_plugins(ContourUiPlugins);

    #[cfg(debug_assertions)]
    app.add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default());

    app.run();
}
