// use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};

type DecathalonScores = HashMap<String, f32>;

pub struct Athlete {
    pub name: String,
    pub country: String,
    pub scores: DecathalonScores,
}

// fn parse_file(input_file_path: &String, output_file_path: &String) -> HashMap<String, Athlete> {
pub fn parse_file(
    input_file_path: &String,
    output_file_path: &String,
) -> Result<HashMap<String, Athlete>, io::Error> {
    let input_file = fs::File::open(&input_file_path)?;
    let buffer = BufReader::new(input_file);
    let mut lines_iter = buffer.lines().peekable();

    let athlete_scores: HashMap<String, Athlete> = HashMap::new();
    let mut event: Option<&str> = None;

    while lines_iter.peek().is_some() {
        let line = lines_iter.next().unwrap()?;
        if line.is_empty() {
            continue;
        }
        println!("{:?}", line);

        // let re_score = Regex::new(r"(\w*) (\w*) ([A-Z]{*}) ([0-9\.]*)").unwrap();

        // let char_vec = (0..height)
        //     .map(|_| lines_iter.next().unwrap())
        //     .collect::<Result<_, _>>()?;
        // athlete_scores.insert(ascii, char_vec);
    }

    Ok(athlete_scores)
}

fn parse_event_line(line: &String) -> Option<&str> {
    let re = Regex::new(r"event (.*)").unwrap();
    let cap = re.captures(line).unwrap();
    let event = cap.get(1).unwrap().as_str();

    Some(event)
}

fn parse_score_line(line: &String) Option<(&str, &str, f32)>{
    let re = Regex::new(r"(\w*) (\w*) ([A-Z]{*}) ([0-9\.]*)").unwrap();
    let cap = re.captures(line).unwrap();
    let first_name = cap.get(1).unwrap().as_str();
    let last_name = cap.get(2).unwrap().as_str();
    let country = cap.get(3).unwrap().as_str();
    let score = cap.get(4).unwrap().as_str();

    let full_name = first_name + last_name;
    Some((full_name, country, score.parse::<f32>()))
}
