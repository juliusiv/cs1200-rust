use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};

pub struct Font {
    pub letters: HashMap<usize, Vec<String>>,
    pub width: usize,
    pub height: usize,
    pub foreground: char,
    pub background: char,
}

pub fn parse_file(font_file_path: &String) -> Result<Font, io::Error> {
    let file = fs::File::open(&font_file_path)?;
    let buffer = BufReader::new(file);
    let mut lines_iter = buffer.lines().peekable();

    let first_line: String = lines_iter.next().unwrap()?;
    let (width, height) = parse_dimension_line(&first_line);
    let mut bitmap_letters: HashMap<usize, Vec<String>> = HashMap::new();

    while lines_iter.peek().is_some() {
        let line = lines_iter.next().unwrap()?;
        let (ascii, _character) = parse_character_line(&line);

        let char_vec = (0..height)
            .map(|_| lines_iter.next().unwrap())
            .collect::<Result<_, _>>()?;
        bitmap_letters.insert(ascii, char_vec);
    }

    let font = Font {
        letters: bitmap_letters,
        width: width,
        height: height,
        foreground: '#',
        background: '.',
    };
    Ok(font)
}

fn parse_dimension_line(line: &String) -> (usize, usize) {
    let split: Vec<&str> = line.split(" ").collect();
    let width = split[0].parse::<usize>().unwrap();
    let height = split[1].parse::<usize>().unwrap();

    (width.into(), height.into())
}

fn parse_character_line(line: &String) -> (usize, &str) {
    let re = Regex::new(r"(\d{1,3}) '(.{1})'").unwrap();

    let cap = re.captures(line).unwrap();
    let ascii = cap.get(1).unwrap().as_str();
    let character = cap.get(2).unwrap().as_str();

    (ascii.parse::<usize>().unwrap(), character)
}
