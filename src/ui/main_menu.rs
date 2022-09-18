use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{epaint::Shadow, style::Margin, Frame, Pos2, Rect, Rounding, Vec2},
    EguiContext,
};

use super::{
    options_menu::OptionsUiState,
    styling::{MENU_BUTTON_FILL, MENU_FILL, MENU_STROKE},
    text::{h1, h2},
};

struct MainUiState {
    show: bool,
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainUiState { show: false })
            .add_system(input_show_menu)
            .add_system(update);
    }
}

fn input_show_menu(
    mut main_menu_state: ResMut<MainUiState>,
    mut options_menu_state: ResMut<OptionsUiState>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_released(KeyCode::Escape) {
        if main_menu_state.show {
            options_menu_state.show = false;
        }
        main_menu_state.show = !main_menu_state.show;
    }
}

fn update(
    mut egui: ResMut<EguiContext>,
    windows: Res<Windows>,
    state: ResMut<MainUiState>,
    mut options_menu_state: ResMut<OptionsUiState>,
    mut exit: EventWriter<AppExit>,
) {
    if state.show {
        let window = windows.get_primary().unwrap();
        let save_exists = true; // TODO actually check for save.

        let center_pos: Pos2 = (window.width() / 6.0, (window.height() / 2.0) - 40.0).into();
        let size: Vec2 = (window.width() / 6.0, window.height() / 2.0).into();
        let button_size = [window.width() / 9.0, window.height() / 10.0];

        bevy_egui::egui::Window::new("MainMenu")
            .title_bar(false)
            .fixed_rect(Rect::from_center_size(center_pos, size))
            .frame(Frame {
                outer_margin: Margin {
                    left: 50.0,
                    ..default()
                },
                inner_margin: Margin::same(40.0),
                fill: MENU_FILL,
                rounding: Rounding::same(8.0),
                shadow: Shadow::big_dark(),
                stroke: MENU_STROKE,
            })
            .show(egui.ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(h1("CONTOUR"));
                    ui.add_space(30.0);

                    if save_exists
                        && ui
                            .add_sized(
                                button_size,
                                bevy_egui::egui::Button::new(h2("Continue")).fill(MENU_BUTTON_FILL),
                            )
                            .clicked()
                    {}
                    ui.add_space(10.0);
                    if ui
                        .add_sized(
                            button_size,
                            bevy_egui::egui::Button::new(h2("New")).fill(MENU_BUTTON_FILL),
                        )
                        .clicked()
                    {}

                    ui.add_space(10.0);

                    if ui
                        .add_sized(
                            button_size,
                            bevy_egui::egui::Button::new(h2("Options")).fill(MENU_BUTTON_FILL),
                        )
                        .clicked()
                    {
                        options_menu_state.show = true
                    }
                    ui.add_space(10.0);

                    if ui
                        .add_sized(
                            button_size,
                            bevy_egui::egui::Button::new(h2("Quit")).fill(MENU_BUTTON_FILL),
                        )
                        .clicked()
                    {
                        exit.send(AppExit);
                    }
                });
            });
    }
}
