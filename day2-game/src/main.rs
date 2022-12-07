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

    pub fn get_score_part2(you: char, outcome: char) -> u32 {
        match you {
            'A' => match outcome { // ROCK
                'X' => LOSS + SCISSOR,
                'Y' => DRAW + ROCK,
                'Z' => WIN + PAPER,
                _ => unreachable!(),
            },
            'B' => match outcome { // PAPER
                'X' => LOSS + ROCK,
                'Y' => DRAW + PAPER,
                'Z' => WIN + SCISSOR,
                _ => unreachable!(),
            },
            'C' => match outcome { // SCISSOR
                'X' => LOSS + PAPER,
                'Y' => DRAW + SCISSOR,
                'Z' => WIN + ROCK,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    pub fn run(input: &str) -> u32 {
        input
            .lines()
            .fold(0, |acc, line| {
                let you = line.chars().nth(0).unwrap();
                let me = line.chars().nth(2).unwrap();
                acc + get_score(you, me)
            })
    }

    pub fn run2(input: &str) -> u32 {
        input
            .lines()
            .fold(0, |acc, line| {
                let you = line.chars().nth(0).unwrap();
                let me = line.chars().nth(2).unwrap();
                acc + get_score_part2(you, me)
            })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    println!("Total Score - Part 1: {}", game::run(&file));

    println!("Total Score - Part 2: {}", game::run2(&file));
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
    fn get_score_part2() {
        assert_eq!(4, game::get_score_part2('A', 'Y'));
        assert_eq!(1, game::get_score_part2('B', 'X'));
        assert_eq!(7, game::get_score_part2('C', 'Z'));
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
