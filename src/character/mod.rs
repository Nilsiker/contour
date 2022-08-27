use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::game::GameState;

use self::{
    enemy::{Enemy, EnemyPlugin},
    player::PlayerPlugin,
};

pub mod enemy;
pub mod player;

#[derive(Component, Inspectable)]
pub struct GameOver(pub bool);

#[derive(Component, Inspectable)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Speed(f32);

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sprite_movement).add_system(sprite_flipping);
    }
}

fn sprite_movement(
    state: Res<State<GameState>>,
    time: Res<Time>,
    mut player: Query<(&MoveDirection, &mut Transform, &Speed), Without<Enemy>>,
    mut enemies: Query<(&MoveDirection, &mut Transform, &Speed), With<Enemy>>,
) {
    for (movement, mut transform, speed) in &mut enemies {
        let move_vector =
            Vec3::new(movement.0.x, movement.0.y, 0.) * time.delta_seconds() * speed.0;
        transform.translation += move_vector;
    }

    match state.current() {
        GameState::InGame => {
            for (movement, mut transform, speed) in &mut player {
                let move_vector =
                    Vec3::new(movement.0.x, movement.0.y, 0.) * time.delta_seconds() * speed.0;
                transform.translation += move_vector;
            }
        }
        _ => (),
    }
}

fn sprite_flipping(mut query: Query<(&mut TextureAtlasSprite, &MoveDirection)>) {
    for (mut sprite, movement) in &mut query {
        if movement.0.x < 0. {
            sprite.flip_x = true
        } else if movement.0.x > 0. {
            sprite.flip_x = false
        }
    }
}
