mod task {
    pub fn parse_start(_input: &str) -> u32 {
        42
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_start() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(5, task::parse_start(&input));
    }
}
