use bevy::prelude::*;
use rand::prelude::random;

use crate::components::grid::{Size, Position};
use crate::components::{food::*, grid::{ARENA_WIDTH, ARENA_HEIGHT}};

pub fn food_spawner(
    mut commands: Commands
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));

}
