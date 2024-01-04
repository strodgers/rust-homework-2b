use core::f32;
use std::env::args;
use std::error::Error;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::convert::TryFrom;
use std::collections::HashMap;

#[derive(Debug)]
enum EnumLine {
    NameAndNumber(String, u32),
    NameOnly(String)
}

#[derive(Default, Debug)]
struct ScoreStruct {
    total_score: u32,
    attended_tests: u32,
    missed_tests: u32,
}

#[allow(dead_code)] // TODO: remember to get rid of this
impl ScoreStruct {
    fn add_score(&mut self, score: u32) {
        self.total_score += score;
        self.attended_tests += 1;
    }

    fn missed_a_test(&mut self) {
        self.missed_tests += 1;
    }

    fn average_score(&self) -> Option<f32> {
        match self.total_score.checked_div(self.attended_tests) {
            Some(_) => Some(self.total_score as f32 / self.attended_tests as f32),
            None => None,
        }
    }

    fn total_score(&self) -> u32 {self.total_score}
    fn attended_tests(&self) -> u32 {self.attended_tests}
    fn missed_tests(&self) -> u32 {self.missed_tests}

}

fn process_scores(lines: Vec<EnumLine>) -> HashMap<String, ScoreStruct> {
    // Go through the scores, adding them to the hashmap
    let mut scores_map: HashMap<String, ScoreStruct> = HashMap::new();
    for line in lines {
        match line {
            // If there's a name and a score, add it to the hashmap
            EnumLine::NameAndNumber(name, score) => {
                scores_map.entry(name).or_default().add_score(score);
            },
            // If there's only a name, assume they missed a test (naughty)
            EnumLine::NameOnly(name) => {
                scores_map.entry(name).or_default().missed_a_test();
            },
        }
    }

    return scores_map;
}

impl TryFrom<&String> for EnumLine {
    type Error = Box<dyn Error>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        // Split the line
        let mut iter = value.split(":");

        // Get the name
        let name = iter.next().ok_or("Expected name string")?;

        // Get the number, if it fails assume name only
        match iter.next() {
            Some(number_str) => 
                return Ok(EnumLine::NameAndNumber(name.to_string(), number_str.parse::<u32>()?)),
            None => return Ok(EnumLine::NameOnly(name.to_string()))
        }
    }
}

fn read_data(fname: &PathBuf) -> Result<Vec<EnumLine>, Box<dyn std::error::Error>> {
    // Read the data and parse into a vector of EnumLine
    let buffread = BufReader::new(File::open(fname)?);
    let mut vec = Vec::new();

    for line_result in buffread.lines() {
        let line = line_result?;
        match EnumLine::try_from(&line) {
            Ok(enum_line) => vec.push(enum_line),
            Err(err) => {
                println!("Error encountered: {}", err);
                return Err(err);
            }
        }
    }

    return Ok(vec);
}
fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename from the command line
    let fname = PathBuf::from(args().nth(1).ok_or("Expected filename")?);

    // Pass filename to read_data
    let result = read_data(&fname);

    // Check if data was read successfully or if an error occurred
    match result {
        Ok(vec_data) => {
            // Print the data
            for (name, score_struct) in &process_scores(vec_data) {
                println!("{:?}: {:?}", name, score_struct);
            }
        },
        Err(err) => {
            println!("An error occurred while reading the data: {}", err);
        }
    }

    Ok(())
}