mod dice;

fn main() {
    println!("I'm going to roll two 6 sided dice, 10 times!");
    for i in 0..10 {
        let result = dice::roll(6, 2);
        println!("roll {}: {}", i+1, result);
    }
}
