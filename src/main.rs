use std::env::args;
use std::error::Error;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::convert::TryFrom;

#[derive(Debug)]
enum EnumLine {
    // Debug for printing

    NameAndNumber(String, u32),
    NameOnly(String)
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

    Ok(vec)
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
            println!("{:?}", vec_data);
        },
        Err(err) => {
            println!("An error occurred while reading the data: {}", err);
        }
    }

    Ok(())
}