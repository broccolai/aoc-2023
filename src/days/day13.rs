use std::iter;

use grid::Grid;
use itertools::Itertools;
use yaah::{aoc, aoc_generator, aoc_test};

use crate::utilities::string::parse_grid;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ash,
    Rock,
}

#[aoc_generator(day13)]
fn generate(input: &'static str) -> Vec<Grid<Token>> {
    let token_matcher = |char| match char {
        '.' => Token::Ash,
        '#' => Token::Rock,
        _ => unreachable!(),
    };

    input
        .split("\n\n")
        .map(|group| parse_grid(group, token_matcher))
        .collect_vec()
}

#[aoc(day13, part1)]
fn part_one(patterns: &[Grid<Token>]) -> usize {
    patterns
        .iter()
        .map(|pattern| find_reflection(pattern, 0))
        .sum()
}

#[aoc(day13, part2)]
fn part_two(patterns: &[Grid<Token>]) -> usize {
    patterns
        .iter()
        .map(|pattern| find_reflection(pattern, 1))
        .sum()
}

fn find_reflection(pattern: &Grid<Token>, tolerance: usize) -> usize {
    let column_search = find_relection_in_lines(pattern.cols() - 1, tolerance, |index| {
        pattern.iter_col(index)
    });

    if let Some(column_result) = column_search {
        return column_result;
    }

    let row_search = find_relection_in_lines(pattern.rows() - 1, tolerance, |index| {
        pattern.iter_row(index)
    });

    if let Some(row_result) = row_search {
        return row_result * 100;
    }

    unreachable!()
}

fn find_relection_in_lines<'a, T>(
    max_distance: usize,
    tolerance: usize,
    tokens_from_index: impl Fn(usize) -> T,
) -> Option<usize>
where
    T: Iterator<Item = &'a Token>,
{
    (0..max_distance)
        .find(|center_index| {
            (0..usize::min(center_index + 1, max_distance - center_index))
                .map(|offset| {
                    iter::zip(
                        tokens_from_index(center_index - offset),
                        tokens_from_index(center_index + offset + 1),
                    )
                    .filter(|&(before, after)| before != after)
                    .count()
                })
                .sum::<usize>()
                .eq(&tolerance)
        })
        .map(|index| index + 1)
}

#[aoc_test(day13, part1)]
fn test_part_one() -> usize { 
    405 
}

#[aoc_test(day13, part2)]
fn test_part_two() -> usize {
    400
}
