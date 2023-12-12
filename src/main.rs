pub mod day6_2023;
use day6_2023::{read_file_to_one_race, read_file_to_races};

fn main() {
    day6();
}

fn day6() {
    day6_part1();
    day6_part2();
}

fn day6_part1() {
    let Ok(races) = read_file_to_races("input_files/input_day_6.txt") else {
        panic!("Failed to read times and distances");
    };

    let mut result: u64 = 1;
    for race in races {
        result = result * race.calculate_number_of_beating_ways();
    }

    println!("Result: {:?}", result);
}

fn day6_part2() {
    let Ok(race) = read_file_to_one_race("input_files/input_day_6.txt") else {
        panic!("Failed to read to race!")
    };
    let result = race.calculate_number_of_beating_ways();
    println!("Result part 2: {:?}", result);
}
