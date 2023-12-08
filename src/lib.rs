use yaah::{aoc_lib, aoc_year};

aoc_year!(2023);

mod utilities;

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
}

aoc_lib!(with_benchmarks);
