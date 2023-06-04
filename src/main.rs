mod dice;
mod board;

fn main() {
    println!("I'm going to roll two 6 sided dice, 10 times!");
    for i in 0..10 {
        let result = dice::roll(6, 2);
        println!("roll {}: {}", i+1, result);
    }

    let tile_1 = board::Tile::new(0);
    let _tile_2 = board::Tile::new(1);
    let tile_3 = board::Tile::new(2);
    let connection = board::Connection::new(board::ConnectionKind::Slide, &tile_3, &tile_1);
    println!("{}", connection);
}
