use ladders_and_slides::{Pawn, BoardComponent, TurnCounter, take_turns};
use bevy::prelude::*;

mod dice;
mod ladders_and_slides;
mod logger;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_pawns)
        .add_startup_system(add_board)
        .add_startup_system(add_turn_counter)
        .add_system(take_turns)
        .run();
}

fn add_pawns(mut commands: Commands) {
    commands.spawn(Pawn::new(0));
    commands.spawn(Pawn::new(1));
}

fn add_board(mut commands: Commands) {
    commands.spawn(BoardComponent::new(40));
}

fn add_turn_counter(mut commands: Commands) {
    commands.spawn(TurnCounter::new());
}
