use ladders_and_slides::LaddersAndSlides;

mod dice;
mod ladders_and_slides;

fn main() {
    let num_tiles = 40;
    let num_players = 2;
    let mut game = LaddersAndSlides::new(num_tiles, num_players);

    loop {
        game.take_turn();
    }
}
