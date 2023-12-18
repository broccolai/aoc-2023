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
    calculate_universe_distances(universe, 2)
}

#[aoc(day11, part2)]
fn part_two(universe: &Universe) -> usize {
    calculate_universe_distances(universe, 1000000)
}

fn calculate_universe_distances(universe: &Universe, expansion_amount: usize) -> usize {
    let (empty_rows, empty_columns) = empty_lines(universe);

    galaxy_points(universe)
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let min = Point::min(a, b);
            let max = Point::max(a, b);
            let empty_relevant_rows = empty_rows
                .iter()
                .filter(|&row| row >= &min.x && row <= &max.x)
                .count();

            let empty_relevant_columns = empty_columns
                .iter()
                .filter(|&column| column >= &min.y && column <= &max.y)
                .count();

            let expansion = (empty_relevant_columns + empty_relevant_rows) * (expansion_amount - 1);

            manhatten_distance(a, b) + expansion
        })
        .sum()
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

const fn manhatten_distance(a: &Point<usize>, b: &Point<usize>) -> usize {
    usize::abs_diff(a.x, b.x) + usize::abs_diff(a.y, b.y)
}

fn is_empty(token: &Token) -> bool {
    token == &Token::Empty
}
