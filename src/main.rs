mod dice;
mod board;

fn main() {
    let board = board::Board::new(20);

    let mut pawn = board::Pawn::new(
        board.tiles.get(0).unwrap()
    );

    let num_ladders = board.connections.iter().filter(|conn| conn.kind == board::ConnectionKind::Ladder).count() as i32;
    let num_slides = board.connections.iter().filter(|conn| conn.kind == board::ConnectionKind::Slide).count() as i32;
    println!(
        "Made a board with {} tiles, {} ladders, and {} slides.",
        board.tiles.len(),
        num_ladders,
        num_slides
    );

    let first_ladder = board.connections.iter().find(|conn| conn.kind == board::ConnectionKind::Ladder).unwrap();

    println!(
        "First ladder moves from tile {} to tile {}.",
         first_ladder.start.position,
         first_ladder.end.position
     );

    println!("Moving pawn from tile {} to tile {}.", pawn.tile.position, first_ladder.start.position);

    pawn.advance(&first_ladder.start, &board.connections);

    println!("Pawn ended up at tile {}!", pawn.tile.position);
}
