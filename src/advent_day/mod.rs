use std::{fs::File, io::BufReader};

pub(crate) mod day1;

pub(crate) struct AdventDay {
    day_num: u8,
}

pub(crate) trait AdventDayProcessor {
    fn new(day_num: u8) -> Self;
    fn read_file(&self) -> BufReader<File>;
    fn execute(&self);
}

impl AdventDayProcessor for AdventDay {
    fn new(day_num: u8) -> Self {
        AdventDay { day_num }
    }

    fn read_file(&self) -> BufReader<File> {
        let file_path = format!("puzzle_input/day{}.txt", self.day_num);
        let file = File::open(file_path).unwrap();
        BufReader::new(file)
    }

    fn execute(&self) {
        let data = self.read_file();

        match self.day_num {
            1 => day1::day1(data),
            // 2 => day2::day2(),
            // 3 => day3::day3(),
            // 4 => day4::day4(),
            // 5 => day5::day5(),
            // 6 => day6::day6(),
            // 7 => day7::day7(),
            // 8 => day8::day8(),
            // 9 => day9::day9(),
            // 10 => day10::day10(),
            // 11 => day11::day11(),
            // 12 => day12::day12(),
            // 13 => day12::day13(),
            // 14 => day12::day14(),
            // 15 => day12::day15(),
            // 16 => day12::day16(),
            // 17 => day12::day17(),
            // 18 => day12::day18(),
            // 19 => day12::day19(),
            // 20 => day12::day20(),
            // 21 => day12::day21(),
            // 22 => day12::day22(),
            // 23 => day12::day23(),
            // 24 => day12::day24(),
            // 25 => day12::day25(),
            _ => println!("Day {} not implemented yet", self.day_num),
        }
    }
}