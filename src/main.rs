mod dice;
mod board;

fn main() {
    println!("I'm going to roll two 6 sided dice, 10 times!");
    for i in 0..10 {
        let result = dice::roll(6, 2);
        println!("roll {}: {}", i+1, result);
    }

    let tiles = vec![
        board::Tile::new(0),
        board::Tile::new(1),
        board::Tile::new(2),
        board::Tile::new(3),
    ];

    let connections = vec![
        board::Connection::new(
            board::ConnectionKind::Ladder,
            tiles.get(1).unwrap(),
            tiles.get(3).unwrap(),
        )
    ];

    let mut pawn = board::Pawn::new(tiles.get(0).unwrap());

    for tile in &tiles {
        println!("{}", tile);
    }

    for conn in &connections {
        println!("{}", conn);
    }

    println!("{}", pawn);

    pawn.advance(tiles.get(1).unwrap(), &connections);

    println!("{}", pawn);
}
