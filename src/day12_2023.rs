use std::fs::read_to_string;

#[derive(PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    Down,
    Up,
}

pub struct Tile {
    value: char,
    energized: bool,
    moving_direction: Vec<Direction>,
}

impl Tile {
    pub fn energize(mut self, incoming_direction: Direction) -> Vec<Direction> {
        self.energized = true;
        let mut outgoing_direction = self.get_outgoing_direction(incoming_direction);
        for direction in self.moving_direction {
            if outgoing_direction.contains(&direction) {
                let index = outgoing_direction
                    .iter()
                    .position(|x| *x == direction)
                    .unwrap();
                outgoing_direction.remove(index);
            }
        }
        outgoing_direction
    }

    fn get_outgoing_direction(&self, incoming_direction: Direction) -> Vec<Direction> {
        match self.value {
            '/' => match incoming_direction {
                Direction::Left => vec![Direction::Up],
                Direction::Up => vec![Direction::Right],
                Direction::Right => vec![Direction::Down],
                Direction::Down => vec![Direction::Left],
            },
            '\\' => match incoming_direction {
                Direction::Left => vec![Direction::Down],
                Direction::Down => vec![Direction::Left],
                Direction::Right => vec![Direction::Up],
                Direction::Up => vec![Direction::Right],
            },
            '|' => match incoming_direction {
                Direction::Right => vec![Direction::Up, Direction::Down],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Down => vec![incoming_direction],
                Direction::Up => vec![incoming_direction],
            },
            '-' => match incoming_direction {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Right => vec![incoming_direction],
                Direction::Left => vec![incoming_direction],
            },
            _ => vec![incoming_direction],
        }
    }
}

pub struct Line {
    tiles: Vec<Tile>,
}

pub fn read_input(path: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn transform_lines(lines: Vec<String>) -> Vec<Line> {
    let mut result = Vec::new();
    for line in lines {
        let mut line_result = Vec::new();
        for c in line.chars() {
            line_result.push(Tile {
                value: c,
                energized: false,
                moving_direction: Vec::new(),
            })
        }
        result.push(Line { tiles: line_result });
    }
    result
}

pub fn process_day_12_part_1() {
    let lines = read_input("input_files/input_day_12_test.txt");
    let lines = transform_lines(lines);
}
