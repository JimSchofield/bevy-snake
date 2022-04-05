use bevy::prelude::*;
use bevy::core::FixedTimestep;

mod components;
mod systems;

use components::food::Food;
use components::snake::*;
use components::grid::*;
use systems::food_spawner::food_spawner;
use systems::movement::*;
use systems::snake_eating::*;
use systems::movement_input::*;

pub struct GameOverEvent;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(
            ClearColor(Color::rgb(0.04, 0.04, 0.04))
        )
        .insert_resource(
            SnakeSegments::default()
        )
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .insert_resource(
            LastTailPosition::default(),
        )
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(
            snake_movement_input
                .label(SnakeMovement::Input)
                .before(SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake_movement.label(SnakeMovement::Movement))
                .with_system(
                    snake_eating
                        .label(SnakeMovement::Eating)
                        .after(SnakeMovement::Movement),
                )
                .with_system(
                    snake_growth
                        .label(SnakeMovement::Growth)
                        .after(SnakeMovement::Eating),
                ),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner)
        )
        .add_system(game_over.after(SnakeMovement::Movement))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(
        OrthographicCameraBundle::new_2d()
    );
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }

        spawn_snake(commands, segments_res);
    }
}
