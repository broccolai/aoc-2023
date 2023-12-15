use yaah::{aoc_lib, aoc_year};

aoc_year!(2023);

mod utilities {
    pub mod string;
    pub mod tuple;
}

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
}

aoc_lib!(with_benchmarks);
