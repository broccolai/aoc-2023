use std::ops::{Add, Div};

use grid::Grid;
use itertools::{unfold, Itertools};
use yaah::aoc;

use crate::utilities::{
    grid::{Position, PositonGrid},
    string::parse_grid,
};

struct Input {
    tokens: Grid<Token>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Token {
    Ground,
    Start,
    Directional(Direction, Direction),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
struct Step {
    direction: Direction,
    position: Position,
}

impl Token {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Directional(Direction::North, Direction::South),
            '-' => Self::Directional(Direction::West, Direction::East),
            'L' => Self::Directional(Direction::North, Direction::East),
            'J' => Self::Directional(Direction::North, Direction::West),
            '7' => Self::Directional(Direction::South, Direction::West),
            'F' => Self::Directional(Direction::South, Direction::East),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }

    fn next_direction(&self, direction: Direction) -> Option<&Direction> {
        match self {
            Self::Ground | Self::Start => None,
            Self::Directional(first, second) => match direction.opposite() {
                dir if dir == *first => Some(second),
                dir if dir == *second => Some(first),
                _ => None,
            },
        }
    }
}

impl Direction {
    const VALUES: [Self; 4] = [Self::North, Self::South, Self::East, Self::West];

    const fn delta(self) -> Position {
        match self {
            Self::North => Position { row: -1, column: 0 },
            Self::East => Position { row: 0, column: 1 },
            Self::South => Position { row: 1, column: 0 },
            Self::West => Position { row: 0, column: -1 },
        }
    }

    const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[aoc(day10, part1)]
fn day10_part1(input: &'static str) -> usize {
    let parsed_input = parse_input(input);

    most_valid_steps_starting_at(&parsed_input.tokens, starting_position(&parsed_input))
        .len()
        .add(1)
        .div(2)
}

fn parse_input(input: &str) -> Input {
    Input {
        tokens: parse_grid(input, Token::from_char),
    }
}

fn starting_position(input: &Input) -> Position {
    input
        .tokens
        .indexed_iter()
        .find(|&(_position, token)| token == &Token::Start)
        .map(|(positon, _)| Position::from_tuple(positon))
        .unwrap()
}

fn most_valid_steps_starting_at(tokens: &Grid<Token>, position: Position) -> Vec<Step> {
    Direction::VALUES
        .iter()
        .map(|&direction| Step {
            direction,
            position,
        })
        .map(|step| most_valid_steps_following(tokens, &step))
        .max_by_key(Vec::len)
        .unwrap_or(Vec::new())
}

fn most_valid_steps_following(tokens: &Grid<Token>, step: &Step) -> Vec<Step> {
    unfold(*step, |state| {
        let position = Position::add(state.position, state.direction.delta())?;
        let token = tokens.get_with_position(position)?;
        let direction = *token.next_direction(state.direction)?;

        *state = Step {
            direction,
            position,
        };

        Some(*state)
    })
    .collect_vec()
}

#[aoc(day10, part2)]
fn day10_part2(input: &'static str) -> i32 {
    let parsed_input = parse_input(input);
    let starting_position = starting_position(&parsed_input);

    let mut step_positions = most_valid_steps_starting_at(&parsed_input.tokens, starting_position)
        .iter()
        .map(|step| step.position)
        .collect_vec();

    step_positions.insert(0, starting_position);
    step_positions.push(starting_position);

    // implementation of the shoelace formula
    let area = step_positions
        .iter()
        .tuple_windows()
        .fold(0i32, |acc, (current, next)| {
            acc + (current.row * next.column) - (current.column * next.row)
        })
        .abs()
        .div(2);

    let step_count = step_positions.len() as i32;

    area - (step_count / 2) + 1
}
