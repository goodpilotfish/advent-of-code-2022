use std::fs;
use std::error::Error;

mod game {
    const WIN: u32 = 6;
    const DRAW: u32 = 3;
    const LOSS: u32 = 0;

    const ROCK: u32 = 1;
    const PAPER: u32 = 2;
    const SCISSOR: u32 = 3;

    pub fn get_score(you: char, me: char) -> u32 {
        match you {
            'A' => match me { // ROCK
                'X' => ROCK + DRAW,
                'Y' => PAPER + WIN,
                'Z' => SCISSOR + LOSS,
                _ => unreachable!(),
            },
            'B' => match me { // PAPER
                'X' => ROCK + LOSS,
                'Y' => PAPER + DRAW,
                'Z' => SCISSOR + WIN,
                _ => unreachable!(),
            },
            'C' => match me { // SCISSOR
                'X' => ROCK + WIN,
                'Y' => PAPER + LOSS,
                'Z' => SCISSOR + DRAW,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub fn run(input: &str) -> u32 {
        let mut sum = 0;
        input
            .lines()
            .for_each(|l| {
                let you = l.chars().nth(0).unwrap();
                let me = l.chars().nth(2).unwrap();
                //println!("{}, {}", you, me);
                sum += get_score(you, me)
            });
        sum
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;

    println!("Total Score: {}", game::run(&file));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_score() {
        assert_eq!(8, game::get_score('A', 'Y'));
        assert_eq!(1, game::get_score('B', 'X'));
        assert_eq!(6, game::get_score('C', 'Z'));
    }

    #[test]
    fn simple() {
        let input = "\
A Y
B X
C Z";
        assert_eq!(15, game::run(&input));
    }
}
