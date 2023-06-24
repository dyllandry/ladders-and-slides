use std::rc::Rc;

use crate::logger::Logger;

pub struct LaddersAndSlides {
    board: Board,
    turn: i32,
    logger: Rc<Logger>
}

impl LaddersAndSlides {
    pub fn new(num_tiles: i32, num_players: i32, logger: &Rc<Logger>) -> Self {
        if num_tiles <= 0 {
            panic!("Cannot create a board with 0 or less tiles.");
        }

        if num_players <= 0 {
            panic!("Cannot create a board with 0 or less players.");
        }

        Self {
            board: Board::new(num_tiles, num_players),
            turn: 0,
            logger: Rc::clone(logger)
        }
    }

    pub fn take_turn(&mut self) {
        let pawn_index = self.turn as usize % self.board.pawns.len();
        let pawn = self.board.pawns.get_mut(pawn_index).unwrap();
        self.logger.log(&format!("It's player {}'s turn.", pawn.player + 1));
        let pawn_start_position = pawn.position;
        let initial_move_distance = crate::dice::roll(6, 1);
        self.logger.log(&format!("Player {} rolled a {}.", pawn.player + 1, initial_move_distance));
        self.logger.log(&format!("Moved player {}'s pawn from {} to {}.", pawn.player + 1, pawn.position, pawn.position + initial_move_distance));
        let final_position = position_after_connections(
            pawn_start_position + initial_move_distance,
            &self.board.connections,
            Some(&self.logger)
        );
        pawn.position = final_position;
        if pawn_start_position + initial_move_distance != final_position {
            self.logger.log(&format!("Player {}'s pawn ended up at {} after connections.", pawn.player + 1, final_position));
        }
        self.logger.log("");
        self.turn += 1;
    }

    pub fn player_positions(&self) -> Vec<i32> {
        self.board.pawns.iter().map(|pawn| pawn.position).collect()
    }

    pub fn connections(&self) -> Vec<Connection> {
        self.board.connections.clone()
    }
}

fn position_after_connections(start_position: i32, connections: &Vec<Connection>, logger: Option<&Rc<Logger>>) -> i32 {
    let mut interim_position = start_position;
    while let Some(connection_to_travel) = connections.iter().find(
        |connection| connection.start == interim_position
    ) {
        if let Some(logger) = logger {
            logger.log(
                format!(
                    "Took connection from {} to {}.",
                    connection_to_travel.start,
                    connection_to_travel.end
                ).as_str()
            );
        }
        interim_position = connection_to_travel.end;
    }
    return interim_position;
}

struct Board {
    num_tiles: i32,
    pawns: Vec<Pawn>,
    connections: Vec<Connection>
}

#[derive(Clone)]
pub struct Connection {
    start: i32,
    end: i32,
}

struct Pawn {
    position: i32,
    player: i32,
}

struct Player {}

impl Pawn {
    pub fn new(player: i32) -> Self {
        Self {
            position: 0,
            player,
        }
    }
}

impl Board {
    pub fn new(num_tiles: i32, num_players: i32) -> Self {
        if num_tiles <= 0 || num_players <= 0 {
            panic!("Cannot make a board with <= 0 tiles or <= 0 players.");
        }

        let mut pawns: Vec<Pawn> = Vec::new();
        for index in 0..num_players {
            pawns.push(Pawn::new(index));
        }

        let mut connections: Vec<Connection> = Vec::new();
        let num_connections = (num_tiles as f32 / 3.0).floor() as i32;
        let mut rng = rand::thread_rng();
        while connections.len() < num_connections as usize {
            // -1 to start tile because you don't want a slide to prevent finishing the game.
            let start_tile = rand::Rng::gen_range(&mut rng, 0..num_tiles-1);
            let end_tile = rand::Rng::gen_range(&mut rng, 0..num_tiles);
            let tiles_are_same = start_tile == end_tile;
            let conn_already_exists = connections.iter().find(|conn| conn.start == end_tile || conn.end == end_tile).is_some();
            if !tiles_are_same && !conn_already_exists {
                connections.push(Connection { start: start_tile, end: end_tile });
            }
        }

        Self {
            num_tiles,
            pawns,
            connections,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod board {
        use super::*;

        #[test]
        fn connections_are_all_between_existing_tiles() {
            // Test many times since tile positions in connections are generated randomly.
            for _ in 0..1000 {
                let num_tiles = 50;
                let board = Board::new(num_tiles, 2);
                for conn in board.connections {
                    assert!(conn.start >= 0 && conn.start < num_tiles-1);
                    assert!(conn.end >= 0 && conn.end < num_tiles);
                }
            }
        }
    }

    mod position_after_connections {
        use super::*;
        #[test]
        fn gets_position_after_many_connections() {
            let connections = vec![
                Connection { start: 3, end: 6 },
                Connection { start: 6, end: 9 },
                Connection { start: 9, end: 1 },
            ];
            let start_position = 3;
            let received_final_position = position_after_connections(start_position, &connections, None);
            let expected_final_position = 1;
            assert_eq!(received_final_position, expected_final_position);
        }
    }
}
