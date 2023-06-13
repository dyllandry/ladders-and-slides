use std::rc::Rc;

pub struct Board {
    pub tiles: Vec<Rc<Tile>>,
    pub connections: Vec<Connection>,
}

impl Board {
    pub fn new(num_tiles: i32) -> Self {
        if num_tiles <= 0 {
            panic!("Cannot create a board with 0 or less tiles.");
        }

        let mut board = Board {
            tiles: Vec::new(),
            connections: Vec::new(),
        };

        for i in 0..num_tiles {
            board.tiles.push(Rc::new(Tile::new(i)));
        }

        let max_num_connections = ((num_tiles as f32) / 2.0).floor() as i32;
        let num_connections = (max_num_connections as f32 / 5.0).floor() as i32;
        let num_ladders = (num_connections as f32 / 2.0).ceil() as i32;
        let num_slides = (num_connections as f32 / 2.0).floor() as i32;

        board.connections = create_board_connections(num_ladders, num_slides, &board.tiles);

        board
    }
}

fn create_board_connections(num_ladders: i32, num_slides: i32, tiles: &Vec<Rc<Tile>>) -> Vec<Connection> {
    let max_num_possible_connections = ((tiles.len() as f32) / 2.0).floor() as i32;
    if num_ladders + num_slides > max_num_possible_connections {
        panic!(
            "Cannot make {} connections with only {} tiles. Maximum possible is {}. Would require at least {} tiles.",
            num_ladders + num_slides,
            tiles.len(),
            max_num_possible_connections,
            (num_ladders + num_slides) * 2
        );
    }

    let num_tiles = tiles.len();
    let mut connections: Vec<Connection> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num_ladders {
        let mut start_tile: Option<&Rc<Tile>> = None;
        let mut end_tile: Option<&Rc<Tile>> = None;
        while start_tile.is_none() && end_tile.is_none() {
            // Grab a random start tile not already part of a connection.
            while start_tile.is_none() {
                let random_tile_index = rand::Rng::gen_range(&mut rng, 0..num_tiles) as usize;
                let tile = tiles.get(random_tile_index).unwrap();
                let existing_conn = connections.iter().find(|conn| Rc::ptr_eq(&conn.start, tile));
                if existing_conn.is_none() {
                    start_tile = Some(tile);
                }
            }

            // Grab a random end tile not already part of a connection.
            while end_tile.is_none() {
                let random_tile_index = rand::Rng::gen_range(&mut rng, 0..num_tiles) as usize;
                let tile = tiles.get(random_tile_index).unwrap();
                let existing_conn = connections.iter().find(|conn| Rc::ptr_eq(&conn.end, tile));
                if existing_conn.is_none() {
                    end_tile = Some(tile);
                }
            }

            // Try again if the tiles we picked don't make a ladder.
            if start_tile.unwrap().position >= end_tile.unwrap().position {
                start_tile = None;
                end_tile = None;
            }
        }


        let conn = Connection::new(ConnectionKind::Ladder, &start_tile.unwrap(), &end_tile.unwrap());
        connections.push(conn);
    }

    for _ in 0..num_slides {
        let mut start_tile: Option<&Rc<Tile>> = None;
        let mut end_tile: Option<&Rc<Tile>> = None;
        while start_tile.is_none() && end_tile.is_none() {
            // Grab a random start tile not already part of a connection.
            while start_tile.is_none() {
                let random_tile_index = rand::Rng::gen_range(&mut rng, 0..num_tiles) as usize;
                let tile = tiles.get(random_tile_index).unwrap();
                let existing_conn = connections.iter().find(|conn| Rc::ptr_eq(&conn.start, tile));
                if existing_conn.is_none() {
                    start_tile = Some(tile);
                }
            }

            // Grab a random end tile not already part of a connection.
            while end_tile.is_none() {
                let random_tile_index = rand::Rng::gen_range(&mut rng, 0..num_tiles) as usize;
                let tile = tiles.get(random_tile_index).unwrap();
                let existing_conn = connections.iter().find(|conn| Rc::ptr_eq(&conn.end, tile));
                if existing_conn.is_none() {
                    end_tile = Some(tile);
                }
            }

            // Try again if the tiles we picked don't make a slide.
            if start_tile.unwrap().position <= end_tile.unwrap().position {
                start_tile = None;
                end_tile = None;
            }
        }

        let conn = Connection::new(ConnectionKind::Slide, &start_tile.unwrap(), &end_tile.unwrap());
        connections.push(conn);
    }

    connections
}

pub struct Pawn {
    pub tile: Rc::<Tile>,
}

impl Pawn {
    pub fn new (tile: &Rc<Tile>) -> Self {
        Self {
            tile: Rc::clone(tile)
        }
    }

    pub fn advance(&mut self, end_tile: &Rc<Tile>, connections: &[Connection]) {
        let mut target_end_tile = end_tile;
        while let Some(connection_to_travel) = connections.iter().find(
            |conn| Rc::ptr_eq(&conn.start, target_end_tile)
        ) {
            target_end_tile = &connection_to_travel.end;
        }
        self.tile = Rc::clone(target_end_tile);
    }
}

impl<'a> std::fmt::Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write![f, "Pawn at tile {}.", self.tile.position]
    }
}

#[derive(Debug)]
pub struct Tile {
    pub position: i32,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write![f, "Tile with position {}", self.position]
    }
}

impl Tile {
    pub fn new(position: i32) -> Self {
        if position < 0 {
            panic!("Tried to create a tile but it's position is negative.");
        }
        Self {
            position
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub start: Rc<Tile>,
    pub end: Rc<Tile>,
    pub kind: ConnectionKind
}

#[derive(PartialEq, Eq, Debug)]
pub enum ConnectionKind {
    Slide,
    Ladder
}

impl std::fmt::Display for ConnectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            ConnectionKind::Slide => "slide",
            ConnectionKind::Ladder => "ladder",
        };
        write!(f, "{}", label)
    }
}

impl Connection {
    pub fn new(kind: ConnectionKind, start: &Rc<Tile>, end: &Rc<Tile>) -> Self {
        if start.position == end.position {
            panic!("Tried to create a connection but it starts and ends at the same place.");
        }

        if kind == ConnectionKind::Slide && start.position < end.position {
            panic!("Tried to create a slide but it goes up.");
        }

        if kind == ConnectionKind::Ladder && start.position > end.position {
            panic!("Tried to create a ladder but it goes down.");
        }

        Self {
            kind,
            start: Rc::clone(start),
            end: Rc::clone(end)
        }
    }
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} from tile {} to tile {}.", self.kind, self.start.position, self.end.position)
    }
}

#[cfg(test)]
mod test {
    mod tile {
        mod new {
            use super::super::super::*;

            #[test]
            #[should_panic]
            fn panics_if_position_is_negative() {
                Tile::new(-1);
            }
        }
    }

    mod connection {
        mod new {
            use super::super::super::*;

            #[test]
            #[should_panic]
            fn panics_if_starts_and_ends_at_same_place() {
                let tile = Rc::new(Tile { position: 0 });
                Connection::new(ConnectionKind::Slide, &tile, &tile);
            }

            #[test]
            #[should_panic]
            fn panics_if_is_slide_but_goes_up() {
                let lower_tile = Rc::new(Tile { position: 0 });
                let higher_tile = Rc::new(Tile { position: 1 });
                Connection::new(ConnectionKind::Slide, &lower_tile, &higher_tile);
            }

            #[test]
            #[should_panic]
            fn panics_if_is_ladder_but_goes_down() {
                let lower_tile = Rc::new(Tile { position: 0 });
                let higher_tile = Rc::new(Tile { position: 1 });
                Connection::new(ConnectionKind::Ladder, &higher_tile, &lower_tile);
            }
        }
    }

    mod pawn {
        mod advance {
            use super::super::super::*;

            #[test]
            fn moves_pawn_to_tile() {
                let start_tile = Rc::new(Tile::new(0));
                let end_tile = Rc::new(Tile::new(1));
                let mut pawn = Pawn::new(&start_tile);
                let connections: Vec<Connection> = Vec::new();
                pawn.advance(&end_tile, &connections);
                assert!(Rc::ptr_eq(&pawn.tile, &end_tile));
            }

            #[test]
            fn follows_connections_until_they_stop() {
                let tile_1 = Rc::new(Tile::new(0));
                let tile_2 = Rc::new(Tile::new(1));
                let tile_3 = Rc::new(Tile::new(2));
                let tile_4 = Rc::new(Tile::new(3));
                let mut pawn = Pawn::new(&tile_1);
                let connections: Vec<Connection> = vec![
                    Connection::new(ConnectionKind::Ladder, &tile_2, &tile_3),
                    Connection::new(ConnectionKind::Ladder, &tile_3, &tile_4),
                ];
                pawn.advance(&tile_2, &connections);
                assert!(Rc::ptr_eq(&pawn.tile, &tile_4));
            }

            #[test]
            fn follows_ladder_and_slide_connections_until_they_stop() {
                let tile_1 = Rc::new(Tile::new(0));
                let tile_2 = Rc::new(Tile::new(1));
                let tile_3 = Rc::new(Tile::new(2));
                let tile_4 = Rc::new(Tile::new(3));
                let mut pawn = Pawn::new(&tile_1);
                let connections: Vec<Connection> = vec![
                    Connection::new(ConnectionKind::Ladder, &tile_2, &tile_4),
                    Connection::new(ConnectionKind::Slide, &tile_4, &tile_3),
                ];
                pawn.advance(&tile_2, &connections);
                assert!(Rc::ptr_eq(&pawn.tile, &tile_3));
            }
        }
    }

    mod create_board_connections {
        use super::super::*;

        fn create_vec_of_rc_tiles(num_tiles: i32) -> Vec<Rc<Tile>> {
            if num_tiles <= 0 {
                panic!("Must create a number of tiles > 0.");
            }
            let mut tiles: Vec<Rc<Tile>> = Vec::new();
            for i in 0..num_tiles {
                tiles.push(Rc::new(Tile::new(i)));
            }
            tiles
        }

        #[test]
        fn returns_requested_number_of_ladders_and_slides() {
            for _ in 0..100 {
                let num_ladders = 2;
                let num_slides = 3;
                let tiles = create_vec_of_rc_tiles(10);
                let received_connections = create_board_connections(num_ladders, num_slides, &tiles);
                let received_num_ladders = received_connections.iter().filter(|conn| conn.kind == ConnectionKind::Ladder).count() as i32;
                let received_num_slides = received_connections.iter().filter(|conn| conn.kind == ConnectionKind::Slide).count() as i32;
                assert_eq!(received_num_slides, num_slides);
                assert_eq!(received_num_ladders, num_ladders);
            }
        }

        #[test]
        fn no_returned_connections_start_or_end_at_the_same_tile() {
            for _ in 0..100 {
                let num_ladders = 5;
                let num_slides = 5;
                let tiles = create_vec_of_rc_tiles(20);
                let received_connections = create_board_connections(num_ladders, num_slides, &tiles);
                for tile in tiles {
                    let mut num_conn_starts = 0;
                    let mut num_conn_ends = 0;
                    for conn in &received_connections {
                        if Rc::ptr_eq(&conn.start, &tile) {
                            num_conn_starts += 1;
                        }
                        if Rc::ptr_eq(&conn.end, &tile) {
                            num_conn_ends += 1;
                        }
                    }
                    assert!(num_conn_starts < 2);
                    assert!(num_conn_ends < 2);
                }
            }
        }

        #[test]
        #[should_panic]
        fn panics_if_requested_connections_are_greater_than_half_the_number_of_tiles_rounded_down() {
            let num_ladders = 2;
            let tiles = create_vec_of_rc_tiles(2);
            create_board_connections(num_ladders, 0, &tiles);
        }
    }
}
