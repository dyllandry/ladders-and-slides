pub struct Tile {
    pub position: i32,
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

pub struct Connection<'a> {
    start: &'a Tile,
    end: &'a Tile,
}

#[derive(PartialEq, Eq)]
pub enum ConnectionKind {
    Slide,
    Ladder
}

impl<'a> Connection<'a> {
    pub fn new<'b: 'a>(kind: ConnectionKind, start: &'b Tile, end: &'b Tile) -> Self {
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
            start,
            end
        }
    }
}

impl<'a> std::fmt::Display for Connection<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection from tile {} to tile {}.", self.start.position, self.end.position)
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
                let tile = Tile { position: 0 };
                Connection::new(ConnectionKind::Slide, &tile, &tile);
            }

            #[test]
            #[should_panic]
            fn panics_if_is_slide_but_goes_up() {
                let lower_tile = Tile { position: 0 };
                let higher_tile = Tile { position: 1 };
                Connection::new(ConnectionKind::Slide, &lower_tile, &higher_tile);
            }

            #[test]
            #[should_panic]
            fn panics_if_is_ladder_but_goes_down() {
                let lower_tile = Tile { position: 0 };
                let higher_tile = Tile { position: 1 };
                Connection::new(ConnectionKind::Ladder, &higher_tile, &lower_tile);
            }
        }
    }
}
