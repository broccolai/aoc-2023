use yaah::{aoc_lib, aoc_year};

aoc_year!(2023);

mod utilities {
    pub mod grid;
    pub mod point;
    pub mod string;
    pub mod tuple;
}

mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
}

aoc_lib!(with_benchmarks);
