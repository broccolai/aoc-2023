use yaah::aoc;
use crate::utilities::tuple_utilities::MapTuple;

struct Card(char);

struct Hand {
    cards: Vec<Card>,
    bid: u32
}

#[aoc(day7, part1)]
fn day7_part1(input: &'static str) -> usize {
    parse_input(input);

    12
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines()    
        .filter_map(|line| line.split_once(' '))
        .map_tuple(parse_cards, |num| num.parse().unwrap())
        .map(|(cards, bid)| Hand { cards, bid })
        .collect()
}

fn parse_cards(source: &str) -> Vec<Card> {
    source.chars().map(|char| Card(char)).collect()
}
