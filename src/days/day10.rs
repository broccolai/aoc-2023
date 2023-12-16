use std::ops::{Add, Div};

use grid::Grid;
use itertools::unfold;
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

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
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

    let starting_position = parsed_input
        .tokens
        .indexed_iter()
        .find(|&(_position, token)| token == &Token::Start)
        .map(|(positon, _)| Position::from_tuple(positon))
        .unwrap();

    positions_around(&parsed_input.tokens, starting_position)
}

fn parse_input(input: &str) -> Input {
    Input {
        tokens: parse_grid(input, Token::from_char),
    }
}

fn positions_around(tokens: &Grid<Token>, position: Position) -> usize {
    Direction::VALUES
        .iter()
        .map(|&direction| Step {
            direction,
            position,
        })
        .map(|step| directional_steps(tokens, &step))
        .max()
        .unwrap_or(0)
}

fn directional_steps(tokens: &Grid<Token>, step: &Step) -> usize {
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
    .count()
    .add(1)
    .div(2)
}
