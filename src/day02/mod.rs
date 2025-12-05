use std::str::FromStr;

#[derive(Debug)]
pub struct Interval {
    pub start: u64,
    pub end: u64,
}

impl FromStr for Interval {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or("Invalid interval format")?;

        Ok(Interval {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        })
    }
}
