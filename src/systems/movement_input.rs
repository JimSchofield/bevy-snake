use bevy::prelude::*;

use crate::components::snake::SnakeHead;
use super::direction::Direction;

pub fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::H) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::J){
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::W) ||keyboard_input.pressed(KeyCode::K)  {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::L) {
            Direction::Right
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    } 
}
