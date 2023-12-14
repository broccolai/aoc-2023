use crate::utilities::tuple::MapTuple;
use itertools::Itertools;
use yaah::aoc;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut comparitor = Ord::cmp(
            &calculate_card_suite_value(self),
            &calculate_card_suite_value(other),
        );

        for (self_card, other_card) in Iterator::zip(self.cards.iter(), other.cards.iter()) {
            comparitor =
                comparitor.then_with(|| Ord::cmp(&card_value(*other_card), &card_value(*self_card)));
        }

        comparitor
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day7, part1)]
fn day7_part1(input: &'static str) -> usize {
    parse_input(input)
        .iter()
        .sorted()
        .rev()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum()
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map_tuple(
            |cards| cards.chars().collect_vec(),
            |num| num.parse().unwrap(),
        )
        .map(|(cards, bid)| Hand { cards, bid })
        .collect()
}

fn calculate_card_suite_value(hand: &Hand) -> u32 {
    let counts = hand
        .cards
        .iter()
        .into_group_map_by(|&card| card)
        .values()
        .map(Vec::len)
        .sorted()
        .rev()
        .collect_vec();

    match counts.as_slice() {
        [5, ..] => 1,
        [4, ..] => 2,
        [3, 2] => 3,
        [3, ..] => 4,
        [2, 2, ..] => 5,
        [2, ..] => 6,
        _ => 7,
    }
}

const fn card_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}
