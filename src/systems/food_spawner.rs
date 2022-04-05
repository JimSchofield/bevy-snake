use bevy::prelude::*;
use rand::prelude::random;

use crate::components::grid::{Size, Position};
use crate::components::snake::SnakeSegment;
use crate::components::{food::*, grid::{ARENA_WIDTH, ARENA_HEIGHT}};

fn create_position() -> Position {
    Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    }
}

fn position_is_taken(
    new_pos: &Position,
    segments: Query<&Position, With<SnakeSegment>>
) -> bool {
    for seg_pos in segments.iter() {
        if seg_pos.x == new_pos.x && seg_pos.y == new_pos.y {
            return true
        }
    }

    false
}

pub fn food_spawner(
    mut commands: Commands,
    segments: Query<&Position, With<SnakeSegment>>
) {
    let new_pos = create_position();

    if position_is_taken(&new_pos, segments) {
        return
    }

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(new_pos)
        .insert(Size::square(0.8));
}
