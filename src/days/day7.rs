use std::{cmp::Reverse, collections::HashMap};

use crate::utilities::tuple_utilities::MapTuple;
use itertools::{Itertools, enumerate};
use yaah::aoc;

const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

#[aoc(day7, part1)]
fn day7_part1(input: &'static str) -> usize {
    parse_input(input)
        .iter()
        .map(|hand| (hand, calculate_hand_rank(hand)))
        .sorted_by_key(|(_hand, rank)| Reverse(*rank))
        .enumerate()
        .map(|(index, (hand, rank))| {
            println!("index, {}, hand {:#?}, rank {}", index, hand, rank);
            (index, (hand, rank))
        })
        .map(|(index, (hand, _rank))| hand.bid * (index + 1))
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

fn calculate_hand_rank(hand: &Hand) -> u32 {
    let counts_by_card = hand
        .cards
        .iter()
        .into_group_map_by(|&card| card)
        .into_iter()
        .map(|(card, values)| (card, values.len() as u32))
        .sorted_by(|(first_card, first_count), (second_card, second_count)| {
            Ord::cmp(second_count, first_count).then_with(|| Ord::cmp(&card_value(*second_card), &card_value(*first_card)))
        })
        .collect_vec();

    // println!("debug {:#?}", counts_by_card);

    let cards = counts_by_card
        .iter()
        .map(|(card, _count)| card)
        .map(|card| card_value(card))
        .collect_vec();

    let counts = counts_by_card
        .iter()
        .map(|(_card, count)| count)
        .collect_vec();

    let mut rank = match counts.as_slice() {
        [5, ..] => 1,          // Five of a kind
        [4, ..] => 2,          // Four of a kind
        [3, 2] => 3, // Full house
        [3, ..] => 4,          // Three of a kind
        [2, 2, ..] => 5, // Two pair
        [2, ..] => 6,          // One pair
        _ => 7,                      // High card
    } * 100000000;

    println!();
    //println!("info {:#?}", counts);
    // println!("slice {:?}", counts_by_card.as_slice());
    println!("hand {:?}, rank {}", hand.cards, rank);
    // Add score based on card values
    for (index, (card, count)) in counts_by_card.iter().enumerate() {
        //println!("index {} card {}, count {}", index, card, count);
        // rank += (15 - card_value(card)) * (10 ^ count);
        //println!("updated rank {}", rank);
    }

    for (index, card) in hand.cards.iter().enumerate() {
        let offset = 10u32.pow((hand.cards.len() - index) as u32);
        let adjusted = offset * card_value(card);

        println!("offset {}, adjusted {}", offset, adjusted);
        rank -= adjusted;
    }

    println!("!!rank {}", rank);

    rank
}

fn calculate_high_card_rank(cards: &[char]) -> u32 {
    cards.iter().map(card_value).sum()
}

fn card_value(card: &char) -> u32 {
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
