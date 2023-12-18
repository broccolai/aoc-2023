#![allow(unstable_name_collisions)]

use std::{collections::HashMap, usize};

use itertools::Itertools;
use yaah::{aoc, aoc_generator};

use crate::utilities::tuple::MapTuple;

#[derive(PartialEq, Copy, Clone)]
enum Token {
    Operational,
    Damaged,
    Unknown,
}

pub struct Row {
    sequence: Vec<Token>,
    sizes: Vec<u16>,
}

impl Row {
    const EXPAND_AMOUNT: usize = 5;

    fn expand(&self) -> Self {
        let expanded_sequence = std::iter::repeat(self.sequence.clone())
            .take(Self::EXPAND_AMOUNT)
            .intersperse_with(|| vec![Token::Unknown])
            .flatten()
            .collect_vec();

        Self {
            sequence: expanded_sequence,
            sizes: self.sizes.repeat(Self::EXPAND_AMOUNT),
        }
    }
}

#[aoc_generator(day12)]
fn generator_one(input: &'static str) -> Vec<Row> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map_tuple(parse_sequence, parse_sizes)
        .map(|(sequence, sizes)| Row { sequence, sizes })
        .collect_vec()
}

fn parse_sequence(sequence: &str) -> Vec<Token> {
    sequence
        .chars()
        .map(|char| match char {
            '.' => Token::Operational,
            '#' => Token::Damaged,
            '?' => Token::Unknown,
            _ => unreachable!(),
        })
        .collect_vec()
}

fn parse_sizes(sizes: &str) -> Vec<u16> {
    sizes
        .split(',')
        .map(str::parse::<u16>)
        .map(Result::unwrap)
        .collect_vec()
}

#[aoc(day12, part1)]
fn part_one(rows: &[Row]) -> usize {
    rows.iter().map(count_arrangements).sum()
}

#[aoc(day12, part2)]
fn part_two(rows: &[Row]) -> usize {
    rows.iter()
        .map(Row::expand)
        .map(|row| count_arrangements(&row))
        .sum()
}

fn count_arrangements(row: &Row) -> usize {
    count_recursively(&row.sequence, &row.sizes, 0, &mut HashMap::new())
}

fn count_recursively(
    template: &[Token],
    counts: &[u16],
    block_size: u16,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(result) = base_case(template, counts, block_size, cache) {
        return result;
    }

    let (next, rest) = template.split_first().unwrap();
    let next_count = counts.first().unwrap();

    let mut sum = 0;

    if matches!(next, Token::Damaged | Token::Unknown) && block_size < *next_count {
        sum += count_recursively(rest, counts, block_size + 1, cache);
    }

    if matches!(next, Token::Operational | Token::Unknown) {
        if block_size == *next_count {
            sum += count_recursively(rest, &counts[1..], 0, cache);
        }

        if block_size == 0 {
            sum += count_recursively(rest, counts, 0, cache);
        }
    }

    if block_size == 0 {
        cache.insert((template.len(), counts.len()), sum);
    }

    sum
}

fn base_case(
    template: &[Token],
    counts: &[u16],
    block_size: u16,
    cache: &HashMap<(usize, usize), usize>,
) -> Option<usize> {
    match (template, counts) {
        (_, []) => Some(
            template
                .iter()
                .all(|x| matches!(x, &Token::Operational | &Token::Unknown))
                .into(),
        ),
        ([], _) => Some((counts == [block_size]).into()),
        _ if block_size == 0 => cache.get(&(template.len(), counts.len())).copied(),
        _ => None,
    }
}
