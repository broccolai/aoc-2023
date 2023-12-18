use crate::utilities::{point::Point, string::parse_grid};
use grid::Grid;
use itertools::Itertools;
use yaah::{aoc, aoc_generator};

type Universe = Grid<Token>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Galaxy,
    Empty,
}

#[aoc_generator(day11)]
fn generator(input: &'static str) -> Universe {
    parse_grid(input, |char| match char {
        '.' => Token::Empty,
        '#' => Token::Galaxy,
        _ => unreachable!(),
    })
}

#[aoc(day11, part1)]
fn part_one(universe: &Universe) -> usize {
    calculate_expanded_distances(universe, 2)
}

#[aoc(day11, part2)]
fn part_two(universe: &Universe) -> usize {
    calculate_expanded_distances(universe, 1_000_000)
}

fn calculate_expanded_distances(universe: &Universe, expansion_factor: usize) -> usize {
    let (empty_rows, empty_columns) = empty_lines(universe);

    galaxy_points(universe)
        .iter()
        .tuple_combinations()
        .map(|(a, b)| calculate_distance(a, b, &empty_rows, &empty_columns, expansion_factor))
        .sum()
}

fn calculate_distance(
    point_a: &Point<usize>,
    point_b: &Point<usize>,
    empty_rows: &[usize],
    empty_columns: &[usize],
    expansion_factor: usize,
) -> usize {
    let min_point = Point::min(point_a, point_b);
    let max_point = Point::max(point_a, point_b);

    let relevant_empty_rows = count_relevant_lines(empty_rows, min_point.x, max_point.x);
    let relevant_empty_columns = count_relevant_lines(empty_columns, min_point.y, max_point.y);

    let expansion = (relevant_empty_columns + relevant_empty_rows) * (expansion_factor - 1);

    Point::manhatten_distance(point_a, point_b) + expansion
}

fn count_relevant_lines(lines: &[usize], min: usize, max: usize) -> usize {
    lines
        .iter()
        .filter(|&&line| line >= min && line <= max)
        .count()
}

fn galaxy_points(universe: &Universe) -> Vec<Point<usize>> {
    universe
        .indexed_iter()
        .filter(|&(_location, token)| token == &Token::Galaxy)
        .map(|(location, _)| location)
        .map(Point::from_tuple)
        .collect_vec()
}

fn empty_lines(universe: &Universe) -> (Vec<usize>, Vec<usize>) {
    let horizontal_lines = universe
        .iter_rows()
        .enumerate()
        .filter(|(_, line)| line.clone().all(is_empty))
        .map(|(index, _)| index)
        .collect_vec();

    let vertical_lines = universe
        .iter_cols()
        .enumerate()
        .filter(|(_, line)| line.clone().all(is_empty))
        .map(|(index, _)| index)
        .collect_vec();

    (horizontal_lines, vertical_lines)
}

fn is_empty(token: &Token) -> bool {
    token == &Token::Empty
}
