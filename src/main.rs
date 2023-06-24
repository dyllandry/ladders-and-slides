use std::rc::Rc;

use ladders_and_slides::LaddersAndSlides;
use logger::Logger;

mod dice;
mod ladders_and_slides;
mod logger;

fn main() {
    // There can only be 1 mutable reference to the logger at any time. Meaning once I pass this to
    // LaddersAndSlides::new( , , logger), I won't be able to use it in main anymore.
    // Is this an opportunity for the interior mutability pattern? I barely know what that is.
    // No it's not.
    // "Interior mutability is a design pattern in Rust that allows you to mutate data even when
    // there are immutable references to that data;"
    // So that doesn't help me. I wanted multiple mutable instances.
    //
    // But wait.. Currently, couldn't anything make their own logger and use it?
    // Does it matter that there would be multiple "File" to the same file?
    // Just wouldn't want to use "Create" every time.
    //
    // Can specify options during file open to append to the file.
    // Look at the actual code example here:
    // https://doc.rust-lang.org/std/fs/struct.File.html#method.options
    //
    // I can just let anything create a logger, and when you call logger.log, it will open the file
    // for appending, append to it, then close? If I can't have multiple File with same append
    // permission at the same time, then I'll need to have each logger open and close the file for
    // each write.
    let logger = Rc::new(Logger::new());

    logger.log("Game started");
    
    let num_tiles = 40;
    let num_players = 2;
    let mut game = LaddersAndSlides::new(num_tiles, num_players, &logger);

    while !game.game_over() {
        game.take_turn();
    }
}
