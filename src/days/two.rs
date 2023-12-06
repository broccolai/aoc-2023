use std::collections::HashMap;
use yaah::aoc;

struct Game {
    identifier: u32,
    draws: Vec<Draw>,
}

struct Draw {
    colors: HashMap<String, u32>,
}

#[aoc(day2, part1)]
fn day2_part1(_input: &'static str) -> u32 {
    let source_draw = &Draw {
        colors: HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ])
    };

    _input.lines()
        .filter_map(parse_game)
        .filter(|game| {
            game.draws.iter().all(|draw| check_draw_is_legal(source_draw, draw))
        })
        .map(|game| game.identifier)
        .sum()
}

fn parse_game(source: &str) -> Option<Game> {
    let (game_identifier, colors) = source.split_once(": ")?;
    let identifier = game_identifier[5..].parse::<u32>().ok()?;

    let draws: Vec<Draw> = colors
        .split("; ")
        .map(parse_draw)
        .collect();

    Some(Game { identifier, draws })
}

fn parse_draw(source: &str) -> Draw {
    let colors: HashMap<String, u32> = source
        .split(", ")
        .filter_map(|color| color.split_once(' '))
        .map(|(amount, name)| (name.to_string(), amount.parse::<u32>().unwrap()))
        .collect();

    Draw { colors }
}

fn check_draw_is_legal(source: &Draw, target: &Draw) -> bool {
    target.colors.iter().all(|(key, value)| {
        source.colors.get(key).map(|source_color| value <= source_color).unwrap_or(false)
    })
}