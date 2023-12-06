pub mod day4;
pub mod day6;
use day6::read_file_to_races;

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

    let mut result: u32 = 1;
    for race in races {
        result = result * race.calculate_number_of_beating_ways();
    }

    println!("Result: {:?}", result);
}

fn day6_part2() {}
