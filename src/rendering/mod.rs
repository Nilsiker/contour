use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

use crate::config::{ConfigUpdateEvent, VideoSettings};

use self::{camera::FollowCameraPlugin, lighting::LightingPlugin};
pub mod camera;
pub mod lighting;

pub struct ContourRenderingPlugins;
impl PluginGroup for ContourRenderingPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(RenderingPlugin)
            .add(FollowCameraPlugin)
            .add(LightingPlugin);
    }
}

struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        let window = app
            .world
            .get_resource::<Windows>()
            .unwrap()
            .get_primary()
            .unwrap();

        app.insert_resource(ResolutionSetting((window.width(), window.height())))
            .insert_resource(VSyncSetting(false))
            .add_startup_system(initiate_window_settings)
            .add_system(update_window_settings)
            .add_system(order_z_entities);
    }
}

fn initiate_window_settings(mut windows: ResMut<Windows>, video: Res<VideoSettings>) {
    let window = windows
        .get_primary_mut()
        .expect("primary window not loaded");

    update_window_helper(window, &video);
}

fn order_z_entities(mut query: Query<&mut Transform, With<OrderedZ>>) {
    for mut transform in &mut query {
        transform.translation.z = -transform.translation.y;
    }
}

fn update_window_settings(
    mut windows: ResMut<Windows>,
    mut events: EventReader<ConfigUpdateEvent>, // TODO make own event..?
    video: Res<VideoSettings>,
) {
    for _ in events.iter() {
        let window = windows.get_primary_mut().unwrap();
        update_window_helper(window, &video);
    }
}

fn update_window_helper(window: &mut Window, video: &Res<VideoSettings>) {
    window.set_present_mode(if video.vsync {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    });

    window.set_mode(if video.fullscreen {
        WindowMode::SizedFullscreen
    } else {
        WindowMode::Windowed
    });

    window.set_resolution(video.resolution.0, video.resolution.1);
}

#[derive(Component)]
pub struct OrderedZ;

pub struct ResolutionSetting((f32, f32));

pub struct VSyncSetting(bool);
