use std::cmp::min;
use bevy::{ecs::component::Component, prelude::Query};

use crate::dice::roll;

pub fn winning_player(pawns: Vec<&Pawn>, board: &Board) -> Option<i32> {
    if let Some(winning_pawn) = pawns.iter().find(|pawn| pawn.position >= board.num_tiles - 1) {
        Some(winning_pawn.player)
    } else {
        None
    }
}

/// Returns a tuple containing the end position and a vec of traveled connections.
fn get_position_after_connections<'a>(
    start_position: i32,
    connections: &'a Vec<Connection>
) -> (i32, Vec<&'a Connection>) {
    let mut interim_position = start_position;
    let mut traveled_connections: Vec<&Connection> = Vec::new();
    while let Some(connection_to_travel) = connections.iter().find(
        |connection| connection.start == interim_position
    ) {
        interim_position = connection_to_travel.end;
        traveled_connections.push(connection_to_travel);
    }
    return (interim_position, traveled_connections);

}

#[derive(Component)]
pub struct Board {
    pub num_tiles: i32,
    pub connections: Vec<Connection>
}


#[derive(Clone, Debug)]
pub struct Connection {
    start: i32,
    end: i32,
}

#[derive(Component, Debug)]
pub struct Pawn {
   pub position: i32,
   pub player: i32,
}

impl Pawn {
    pub fn new(player: i32) -> Self {
        Self {
            position: 0,
            player,
        }
    }
}

impl Board {
    pub fn new(num_tiles: i32) -> Self {
        if num_tiles <= 0 {
            panic!("Cannot make a board with <= 0 tiles or <= 0 players.");
        }

        let mut connections: Vec<Connection> = Vec::new();
        let num_connections = (num_tiles as f32 / 3.0).floor() as i32;
        let mut rng = rand::thread_rng();
        while connections.len() < num_connections as usize {
            // -1 to start tile because you don't want a slide to prevent finishing the game.
            let start_tile = rand::Rng::gen_range(&mut rng, 0..num_tiles-1);
            let end_tile = rand::Rng::gen_range(&mut rng, 0..num_tiles);
            let tiles_are_same = start_tile == end_tile;
            let conn_already_exists = connections.iter().find(|conn| conn.start == start_tile || conn.end == end_tile).is_some();
            let mirror_conn_exists = connections.iter().find(|conn| conn.start == end_tile || conn.end == start_tile).is_some();
            if !tiles_are_same && !conn_already_exists && !mirror_conn_exists {
                connections.push(Connection { start: start_tile, end: end_tile });
            }
        }

        Self {
            num_tiles,
            connections,
        }
    }
}

#[derive(Component, Debug)]
pub struct TurnCounter {
    pub turn: i32
}

impl TurnCounter {
    pub fn new() -> Self {
        Self {
            turn: 0,
        }
    }
}

pub fn take_turns(
    board_query: Query<&Board>,
    mut pawn_query: Query<&mut Pawn>,
    mut turn_counter_query: Query<&mut TurnCounter>,
) {
    let board = board_query.single();
    if winning_player(pawn_query.iter().collect(), board).is_some()  {
        return;
    }

    let mut turn_counter = turn_counter_query.single_mut();
    let next_player_index = turn_counter.turn % pawn_query.iter().len() as i32;
    let mut next_pawn = pawn_query
        .iter_mut()
        .find(|p| p.player == next_player_index).unwrap();

    let initial_move_distance = roll(6, 2);
    println!(
        "Player {} rolled a {}.",
        next_pawn.player + 1,
        initial_move_distance
    );
    let position_after_initial_move = min(next_pawn.position + initial_move_distance, board.num_tiles - 1);
    println!(
        "Player {}'s pawn moved from {} to {}.",
        next_pawn.player + 1,
        next_pawn.position,
        position_after_initial_move
    );

    let (pawn_position_after_connections, traveled_connections) = get_position_after_connections(
        position_after_initial_move,
        &board.connections
    );
    for traveled_conn in traveled_connections {
        println!(
            "Player {}'s pawn took a connection from {} to {}.",
            next_pawn.player + 1,
            traveled_conn.start,
            traveled_conn.end
        );
    }

    next_pawn.position = min(pawn_position_after_connections, board.num_tiles - 1);

    if let Some(winning_player) = winning_player(pawn_query.iter().collect(), board) {
        println!("Player {} won!", winning_player + 1);
        return;
    } else {
        turn_counter.turn += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod board {
        use std::collections::HashSet;

        use super::*;

        #[test]
        /// This is a connection that is the same as another but reversed.
        /// For example, a connection from 1 -> 5 and another from 5 -> 1
        fn no_mirror_connections_exist() {
            // Test many times since tile positions in connections are generated randomly.
            for i in 0..1000 {
                let num_tiles = 50;
                let board = Board::new(num_tiles);

                for conn in &board.connections {
                    let mirror_connection = &board.connections.iter().find(
                        |conn_2| conn.start == conn_2.end && conn.end == conn_2.start
                    );
                    if let Some(mirror_connection) = mirror_connection {
                        panic!(
                            "Mirror connection exists between connections: {:?} and {:?} on test iteration {}.",
                            &conn,
                            &mirror_connection,
                            i
                        );
                    }
                }
            }
        }

        #[test]
        fn no_connections_start_at_same_position() {
            // Test many times since tile positions in connections are generated randomly.
            for i in 0..1000 {
                let num_tiles = 50;
                let board = Board::new(num_tiles);

                let mut start_positions = HashSet::new();
                for conn in board.connections {
                    if start_positions.contains(&conn.start) {
                        panic!(
                            "Start positions already includes {}. Failed on test iteration {}. Start positions are {:?}",
                            &conn.start,
                            i,
                            start_positions
                        );
                    } else {
                        start_positions.insert(conn.start);
                    }
                }
            }
        }

        #[test]
        fn connections_are_all_between_existing_tiles() {
            // Test many times since tile positions in connections are generated randomly.
            for _ in 0..1000 {
                let num_tiles = 50;
                let board = Board::new(num_tiles);
                for conn in board.connections {
                    assert!(conn.start >= 0 && conn.start < num_tiles-1);
                    assert!(conn.end >= 0 && conn.end < num_tiles);
                }
            }
        }
    }

    mod get_position_after_connections {
        use super::*;

        #[test]
        fn returns_correct_position() {
            let connections = vec![
                Connection { start: 3, end: 6 },
                Connection { start: 6, end: 9 },
                Connection { start: 9, end: 1 },
            ];
            let start_position = 3;
            let (received_final_position, _) = get_position_after_connections(start_position, &connections);
            let expected_final_position = 1;
            assert_eq!(received_final_position, expected_final_position);
        }

        #[test]
        fn returns_traveled_connections() {
            let connections_to_travel = vec![
                Connection { start: 3, end: 6 },
                Connection { start: 6, end: 9 },
                Connection { start: 9, end: 1 },
            ];
            let connection_not_to_travel = vec![
                Connection { start: 100, end: 200 },
            ];
            let mut all_connections = Vec::new();
            all_connections.extend(connections_to_travel.clone().into_iter());
            all_connections.extend(connection_not_to_travel.clone().into_iter());

            let start_position = 3;
            let (_, traveled_connections) = get_position_after_connections(start_position, &all_connections);

            for conn_to_travel in connections_to_travel {
                let traveled_conn = traveled_connections.iter().find(
                    |traveled_conn|
                        conn_to_travel.start == traveled_conn.start
                        && conn_to_travel.end == traveled_conn.end
                );
                assert!(traveled_conn.is_some());
            }

            for conn_not_to_travel in connection_not_to_travel {
                let traveled_conn = traveled_connections.iter().find(
                    |traveled_conn|
                        conn_not_to_travel.start == traveled_conn.start
                        && conn_not_to_travel.end == traveled_conn.end
                );
                assert!(traveled_conn.is_none());
            }
        }
    }
}
