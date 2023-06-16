I think I can program this better.

```
let player_1 = player::new();
let player_2 = player::new();

let ladders_and_snakes = LaddersAndSnakes::new(vec![&player_1, &player_2]);

// it knows which player's turn it is
// it will roll the dice for them
// it will move the pawn correctly
ladders_and_snakes.take_turn();

struct LaddersAndSnakes {
	// This is domain stuff
	board,

	// Business logic
	fn take_turn() {}

	fn next_player()
}

struct board {
	num_tiles: 99
	ladders: ladder[]
	slides: slide[],
	pawns: pawn[]

	// domain
	fn new(num_tiles, players)
}

struct pawn {
	position: i32,
	player: &Player
}

struct player {}
```
