use std::env::args;
use std::error::Error;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::convert::TryFrom;

enum EnumLine {
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
        let number_str = iter.next();
        if number_str.is_none() {
            return Ok(EnumLine::NameOnly(name.to_string()));
        }

        // Parse the number
        let number = number_str.ok_or("Expected a number string")?
            .parse::<u32>();
        if number.is_err() {
            return Ok(EnumLine::NameOnly(name.to_string()));
        }
        
        Ok(EnumLine::NameAndNumber(name.to_string(), number?))
    }

}
fn main() -> Result<(), Box<dyn Error>> {
    let fname = PathBuf::from(args().nth(1).ok_or("Expected filename")?);
    let buffread = BufReader::new(File::open(&fname)?);
   
    for line_result in buffread.lines() {
        let line = line_result?;
        let enum_line = EnumLine::try_from(&line)?;
        match enum_line {
            EnumLine::NameAndNumber(name, number) => println!("Name: {}, Number: {}", name, number),
            EnumLine::NameOnly(name) => println!("Name: {}", name)
        }
    }

    Ok(())
}