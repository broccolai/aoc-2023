use rayon::prelude::*;
use yaah::aoc;

const _SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    conversions: Vec<Conversion>,
}

#[derive(Debug)]
struct Conversion {
    steps: Vec<ConversionStep>,
}

#[derive(Debug)]
struct ConversionStep {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

#[aoc(day5, part1)]
fn day5_part1(input: &'static str) -> u32 {
    let almanac = Almanac {
        seeds: parse_seeds(input),
        conversions: parse_conversions(input),
    };

    *process_almanac(&almanac).iter().min().unwrap()
}

fn parse_seeds(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("seeds:")
        .split_ascii_whitespace()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect()
}

fn parse_conversions(input: &str) -> Vec<Conversion> {
    let mut conversions: Vec<Conversion> = Vec::new();
    let mut steps: Vec<ConversionStep> = Vec::new();

    for line in input.lines().skip(2) {
        if line.is_empty() {
            conversions.push(Conversion { steps });
            steps = Vec::new();
            continue;
        }

        if line.contains(':') {
            continue;
        }

        let parsed_numbers: Vec<u32> = line
            .split_ascii_whitespace()
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect();

        steps.push(ConversionStep {
            destination_start: parsed_numbers[0],
            source_start: parsed_numbers[1],
            length: parsed_numbers[2],
        });
    }

    if !steps.is_empty() {
        conversions.push(Conversion { steps });
    }

    conversions
}

fn process_almanac(almanac: &Almanac) -> Vec<u32> {
    let Almanac { seeds, conversions } = almanac;
    let processed_seeds: Vec<u32> = seeds
        .par_iter()
        .map(|seed| {
            let mut processed_seed = *seed;

            for conversion in conversions {
                for step in &conversion.steps {
                    if let Some(value) = apply_step_to_seed(step, &processed_seed) {
                        processed_seed = value;
                        break;
                    }
                }
            }

            processed_seed
        })
        .collect();

    processed_seeds
}

fn apply_step_to_seed(step: &ConversionStep, seed: &u32) -> Option<u32> {
    if &step.source_start > seed {
        return None;
    }

    let adjusted = seed - step.source_start;

    if adjusted > step.length {
        return None;
    }

    Some(step.destination_start + adjusted)
}

// todo: really poor implementation that just bruce forces through all seeds
//       run time is still only ~20s on my machine but this could be a lot better.
//       iterating over the steps instead of seeds and applying their transformations
//       would be faster. ideally you would start from the lowest resulting conversion
//       and go up until you found a seed that matched in your source.
#[aoc(day5, part2)]
fn day5_part2(input: &'static str) -> u32 {
    let almanac = Almanac {
        seeds: parse_seeds_as_ranges(input),
        conversions: parse_conversions(input),
    };

    *process_almanac(&almanac).iter().min().unwrap()
}

fn parse_seeds_as_ranges(input: &str) -> Vec<u32> {
    let raw_numbers = parse_seeds(input);

    raw_numbers
        .chunks_exact(2)
        .flat_map(|values| {
            let start = values[0];
            let length = values[1];

            start..start + length
        })
        .collect()
}
