use std::rc::Rc;

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

pub struct Connection {
    pub start: Rc<Tile>,
    pub end: Rc<Tile>,
    pub kind: ConnectionKind
}

#[derive(PartialEq, Eq)]
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
}
