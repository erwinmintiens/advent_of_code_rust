use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct Race {
    total_time: u32,
    distance_to_beat: u32,
}

impl Race {
    pub fn calculate_number_of_beating_ways(&self) -> u32 {
        let mut number_of_non_beating_ways: u32 = 0;
        for i in 0..self.total_time {
            if i * (self.total_time - i) < self.distance_to_beat {
                number_of_non_beating_ways += 1;
            } else {
                break;
            }
        }
        self.total_time - 2 * number_of_non_beating_ways + 1
    }
}

pub fn read_file_to_races(path: &str) -> io::Result<Vec<Race>> {
    let path = Path::new(path);
    let file = File::open(&path)?;

    let mut time_values: Vec<u32> = Vec::new();
    let mut distance_values: Vec<u32> = Vec::new();

    let mut races: Vec<Race> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;

        let words: Vec<&str> = line.split_whitespace().collect();

        if words[0] == "Time:" {
            for i in 1..5 {
                time_values.push(words[i].parse::<u32>().unwrap());
            }
        }
        if words[0] == "Distance:" {
            for i in 1..5 {
                distance_values.push(words[i].parse::<u32>().unwrap());
            }
        }
    }

    for (index, _) in time_values.iter().enumerate() {
        races.push(Race {
            total_time: time_values[index],
            distance_to_beat: distance_values[index],
        });
    }

    Ok(races)
}
