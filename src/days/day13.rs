use grid::Grid;
use itertools::Itertools;
use yaah::{aoc, aoc_generator};

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
    patterns.iter().map(find_reflection).sum()
}

fn find_reflection(pattern: &Grid<Token>) -> usize {
    let (pattern_rows, pattern_columns) = pattern.size();

    let column_search = find_relection_in_lines(pattern_columns - 1, |before, after| {
        itertools::equal(pattern.iter_col(before), pattern.iter_col(after))
    });

    if let Some(column_result) = column_search {
        return column_result;
    }

    let row_search = find_relection_in_lines(pattern_rows - 1, |before, after| {
        itertools::equal(pattern.iter_row(before), pattern.iter_row(after))
    });

    if let Some(row_result) = row_search {
        return row_result * 100;
    }

    unreachable!()
}

fn find_relection_in_lines(
    max_distance: usize,
    is_mirror_match: impl Fn(usize, usize) -> bool,
) -> Option<usize> {
    (0..max_distance)
        .find(|center_index| {
            let mut edge_distances = 0..usize::min(center_index + 1, max_distance - center_index);

            edge_distances
                .all(|offset| is_mirror_match(center_index - offset, center_index + offset + 1))
        })
        .map(|index| index + 1)
}

#[test]
fn test() {
    let sample = include_str!("../../input/2023_sample/day13.txt");
    let source = generate(sample);
    let result = part_one(&source);

    assert_eq!(result, 405);
}
