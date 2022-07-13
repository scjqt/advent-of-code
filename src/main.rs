use macros::years;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    time::Instant,
};

mod year2019;
mod year2020;
mod year2021;
mod year2022;

fn main() {
    if run().is_none() {
        println!("invalid arguments");
    }
}

fn run() -> Option<()> {
    let mut args = env::args();
    let mut arg = args.nth(1)?;
    let test = &arg == "test";
    if test {
        arg = args.next()?;
    }
    let year = parse_year(arg)?;
    let day = parse_day(args.next()?)?;
    let input = if let Some(input) = if test {
        test_input()
    } else {
        get_input(year, day)
    } {
        input
    } else {
        println!("input not found");
        return Some(());
    };
    if let Some(part) = args.next() {
        let part = parse_part(part)?;
        if day == 25 && part == 2 {
            return None;
        }
        title(year, day);
        run_part(year, day, part, &input);
    } else {
        title(year, day);
        run_part(year, day, 1, &input);
        if day < 25 {
            run_part(year, day, 2, &input);
        }
    }
    Some(())
}

fn run_part(year: u8, day: u8, part: u8, input: &[String]) {
    println!("part {}:", part);
    let start = Instant::now();
    if years!(19..=22) {
        println!("({}ms)", start.elapsed().as_nanos() as f64 / 1_000_000f64);
    } else {
        println!("not yet implemented");
    }
}

fn title(year: u8, day: u8) {
    let year = year as u16 + 2000;
    println!("{} day {}", year, day);
}

fn parse_year(year: String) -> Option<u8> {
    let mut year: u16 = year.parse().ok()?;
    if year > 2000 {
        year -= 2000;
    }
    if year < 15 || year > 255 {
        return None;
    }
    Some(year as u8)
}

fn parse_day(day: String) -> Option<u8> {
    let day: u8 = day.parse().ok()?;
    if day == 0 || day > 25 {
        return None;
    }
    Some(day)
}

fn parse_part(part: String) -> Option<u8> {
    let part: u8 = part.parse().ok()?;
    if part == 0 || part > 2 {
        return None;
    }
    Some(part)
}

fn get_input(year: u8, day: u8) -> Option<Vec<String>> {
    let year = (year as u16 + 2000).to_string();
    let day = if day < 10 {
        format!("0{}", day)
    } else {
        day.to_string()
    };
    read_input(Path::new(&year).join(day))
}

fn test_input() -> Option<Vec<String>> {
    read_input(Path::new("test").to_path_buf())
}

fn read_input(path: PathBuf) -> Option<Vec<String>> {
    BufReader::new(File::open(Path::new("inputs").join(path)).ok()?)
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .ok()
}
