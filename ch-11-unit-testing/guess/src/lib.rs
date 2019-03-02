pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Value must be greater than 1");
        } else if value > 100 {
            panic!("Value must be less than 100");
        }

        Guess {
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Value must be less than 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
