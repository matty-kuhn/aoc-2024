use super::Day;

pub struct Day22 {
    input: String,
}

impl Day22 {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl Day for Day22 {
    fn part1(&self) -> String {
        // step 1: secret = ((secret << 6) ^ secret) % 16777216
        // step 2: secret = ((secret >> 5) ^ secret) % 16777216
        // step 3: secret = ((secret  << 11) ^ secret) % 16777216
        self.input
            .lines()
            .fold(0, |acc, num| {
                let mut secret = num.parse::<usize>().unwrap();
                for _ in 0..2000 {
                    secret = ((secret << 6) ^ secret) % 16777216;
                    secret = ((secret >> 5) ^ secret) % 16777216;
                    secret = ((secret << 11) ^ secret) % 16777216;
                }
                // dbg!(secret);

                acc + secret
            })
            .to_string()

        // todo!()
    }

    fn part2(&self) -> String {
        todo!()
    }
}
