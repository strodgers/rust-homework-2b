use std::env::args;
use std::error::Error;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::convert::TryFrom;
use std::fmt;

// Enum for the raw instructions
enum RawInstruction {
    IncrementPointer,     // >
    DecrementPointer,     // <
    IncrementByte,        // +
    DecrementByte,        // -
    OutputByte,           // .
    InputByte,            // ,
    ConditionalForward,   // [
    ConditionalBackward,  // ]
}

// Implement a from_char method for RawInstruction
impl RawInstruction {
    fn from_char(c: &char) -> Option<RawInstruction> {
        match c {
            '>' => Some(RawInstruction::IncrementPointer),
            '<' => Some(RawInstruction::DecrementPointer),
            '+' => Some(RawInstruction::IncrementByte),
            '-' => Some(RawInstruction::DecrementByte),
            '.' => Some(RawInstruction::OutputByte),
            ',' => Some(RawInstruction::InputByte),
            '[' => Some(RawInstruction::ConditionalForward),
            ']' => Some(RawInstruction::ConditionalBackward),
            _ => None,
        }
    }
}

// Corresponding display strings
impl fmt::Display for RawInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RawInstruction::IncrementPointer => write!(f, "Increment Pointer (>)"),
            RawInstruction::DecrementPointer => write!(f, "Decrement Pointer (<)"),
            RawInstruction::IncrementByte => write!(f, "Increment Byte (+)"),
            RawInstruction::DecrementByte => write!(f, "Decrement Byte (-)"),
            RawInstruction::OutputByte => write!(f, "Output Byte (.)"),
            RawInstruction::InputByte => write!(f, "Input Byte (,)"),
            RawInstruction::ConditionalForward => write!(f, "Conditional Forward ([)"),
            RawInstruction::ConditionalBackward => write!(f, "Conditional Backward (])"),
        }
    }
}

// Probably unnecessary but I like it
impl TryFrom<&char> for RawInstruction {
    type Error = Box<dyn Error>;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        return RawInstruction::from_char(value).ok_or("Invalid character".into());
    }
}

// Struct for the human readable instructions which includes a RawInstruction and the line and column index
struct HumanReadableInstruction {
    instruction: RawInstruction,
    line: usize,
    column: usize,
}

impl HumanReadableInstruction {
    fn new(instruction: RawInstruction, line: usize, column: usize) -> Self {
        return HumanReadableInstruction {
            instruction,
            line,
            column,
        }
    }
}

// Nice display string
impl fmt::Display for HumanReadableInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, ":{}:{}] {}", self.line, self.column, self.instruction);
    }
}

fn read_data(fname: &PathBuf) -> Result<Vec<HumanReadableInstruction>, Box<dyn std::error::Error>> {
    // Read the data and parse into a vector of HumanReadableInstruction
    let buffread = BufReader::new(File::open(fname)?);
    let mut vec = Vec::new();

    // Go through each line
    for (line_idx, line_result) in buffread.lines().enumerate() {
        let line = line_result?;
        
        // Go through each character
        for (col_idx, c) in line.chars().enumerate() {
            match RawInstruction::try_from(&c) {
                Ok(instruction) => {
                    // I am adding 1 to the column and row index here for human readability,
                    // although that does make my output column index 1 off the example given
                    // in the homework description...
                    vec.push(HumanReadableInstruction::new(instruction, line_idx + 1, col_idx + 1));
                    // println!("Valid character {} at line {} column {}",  c, line_idx + 1, col_idx + 1);
                },
                Err(_) => {
                // println!("Invalid character {} at line {} column {}", c, line_idx + 1, col_idx + 1);
                }
            }
        }
    }

    return Ok(vec);
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename from the command line
    let fname = args().nth(1).ok_or("Expected filename")?;
    let fbuf = PathBuf::from(&fname);

    // Pass filename to read_data
    let result = read_data(&fbuf);

    // I do not like that the opening square bracket is here but the closing square bracket is 
    // in the HumanReadableInstruction struct. However it is late now
    for instruction in result? {
        println!("[{}{}", fname, instruction);
    }

    Ok(())
}