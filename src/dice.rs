pub fn roll(sides: i32, rolls: i32) -> i32 {
    if rolls <= 0 || sides <= 0 {
        panic!("Parameters \"rolls\" or \"sides\" cannot be 0 or less. rolls: ({}) sides: ({})", rolls, sides);
    }

    let mut sum = 0;
    for _ in 0..rolls {
        let mut rng = rand::thread_rng();
        sum += rand::Rng::gen_range(&mut rng, 1..sides+1);
    }
    sum
}

#[cfg(test)]
mod test {
    mod roll {
        use super::super::*;

        #[test]
        fn always_returns_a_result_no_greater_than_the_dices_number_of_sides_times_the_number_of_rolls() {
            let sides = 6;
            let rolls = 5;
            for _ in 0..1000 {
                let max = sides * rolls;
                let result = roll(sides, rolls);
                assert!(result <= max);
            }
        }

        #[test]
        fn always_returns_a_result_greater_or_equal_to_the_number_of_dice() {
            let sides = 6;
            let rolls = 5;
            for _ in 0..1000 {
                let result = roll(sides, rolls);
                assert!(result >= 5);
            }
        }

        #[test]
        #[should_panic]
        fn panics_if_rolls_is_0_or_less() {
            roll(0, 1);
        }

        #[test]
        #[should_panic]
        fn panics_if_sides_is_0_or_less() {
            roll(1, 0);
        }
    }
}
