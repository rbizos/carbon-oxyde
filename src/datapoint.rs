use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Datapoint {
    path: String,
    timestamp: u64,
    value: i64,
}

impl Datapoint {
    pub fn new(path: String, timestamp: u64, value: i64) -> Datapoint {
        return Datapoint {
            path,
            timestamp,
            value,
        };
    }
    pub fn from_string(input: &str) -> Result<Datapoint, Box<dyn Error>> {
        let mut split = input.split(' ');
        let path = split.next().ok_or("no path")?.to_string();
        let timestamp = split.next().ok_or("no timestamp")?.parse::<u64>()?;
        let value = split.next().ok_or("no value")?.parse::<i64>()?;
        return Ok(Datapoint {
            path,
            timestamp,
            value,
        });
    }
}
