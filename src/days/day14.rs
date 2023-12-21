use grid::Grid;
use yaah::{aoc, aoc_generator, aoc_test};

use crate::utilities::{grid::UtilityGrid, string::parse_grid};

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Empty,
    Cube,
    Round,
}

#[aoc_generator(day14, part1)]
fn generate(input: &'static str) -> Grid<Token> {
    parse_grid(input, |char| match char {
        '.' => Token::Empty,
        '#' => Token::Cube,
        'O' => Token::Round,
        _ => unreachable!(),
    })
}

#[aoc(day14, part1)]
fn part_one(input: &Grid<Token>) -> usize {
    count_rock_weight(&slide_rocks_north(input))
}

#[aoc_test(day14, part1)]
fn test_part_one() -> usize {
    136
}

fn slide_rocks_north(source: &Grid<Token>) -> Grid<Token> {
    let mut result = source.clone();

    for column_index in 0..result.cols() {
        for row_index in 0..result.rows() {
            let token = *result.get(row_index, column_index).unwrap();

            if !matches!(token, Token::Round) {
                continue;
            }

            let mut row_cursor = row_index;

            while row_cursor > 0 {
                let row_above = row_cursor - 1;

                match result.get(row_above, column_index) {
                    Some(Token::Empty) => {
                        result.set(row_cursor, column_index, Token::Empty);
                        result.set(row_above, column_index, Token::Round);
                    }
                    _ => break,
                };

                row_cursor -= 1;
            }
        }
    }

    result
}

fn count_rock_weight(grid: &Grid<Token>) -> usize {
    let total_rows = grid.rows();

    grid.indexed_iter()
        .filter(|&(_, value)| matches!(value, Token::Round))
        .map(|((row, _), _)| total_rows - row)
        .sum()
}
