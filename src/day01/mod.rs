use std::str::FromStr;

#[derive(Debug)]
pub struct Input {
    pub direction: char,
    pub steps: u32,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let direction = chars.next().unwrap();
        let steps = chars.as_str().parse().unwrap();
        Ok(Input { direction, steps })
    }
}
