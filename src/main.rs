use std::fs::{self,File};
use std::io::{self, BufRead};
use std::num::NonZero;
use std::path::Path;
use std::env;
use std::process;

use crate::permanganate::builder::SquareBoardBuilder;
pub use permanganate;
use permanganate::Builder;
use permanganate::Location;
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct LetterPair {
    letter: char,
    positions: [Position; 2],
}

fn get_line_count(filename: &str) -> usize {
    fs::read_to_string(filename).unwrap().lines().count()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        process::exit(1);
    }

    let filename: String = args[1].clone();
    let path: &Path = Path::new(&filename); // Replace with the actual path to your file
    let line_count: usize = get_line_count(&filename);
    let file: File = File::open(path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut letter_locations: HashMap<char, Vec<Position>> = HashMap::new();
    let mut row_index: usize = 0;

    for line_result in reader.lines() {
        let line: String = line_result?;
        for (col_index, char) in line.chars().enumerate() {
            if char != '.' {
                let position: Position = Position {
                    row: row_index,
                    col: col_index,
                };
                letter_locations.entry(char).or_insert(Vec::new()).push(position);
            }
        }
        row_index += 1;
    }

    let mut letter_pairs: Vec<LetterPair> = Vec::new();
    for (letter, positions) in letter_locations.iter() {
        if positions.len() == 2 {
            letter_pairs.push(LetterPair {
                letter: *letter,
                positions: [positions[0], positions[1]],
            });
        } else if !positions.is_empty() {
            println!(
                "Warning: Letter '{}' appears {} times, not exactly twice.",
                letter,
                positions.len()
            );
        }
    }

    let mut board: SquareBoardBuilder =
        SquareBoardBuilder::with_dims((NonZero::new(line_count).unwrap(), NonZero::new(line_count).unwrap()));
    for data in letter_pairs {
        board.add_termini(
            data.letter,
            (Location(data.positions[0].col, data.positions[0].row), Location(data.positions[1].col, data.positions[1].row)),
        );
    }

    let solved: permanganate::Board<permanganate::shape::SquareStep> = board.build().unwrap();
    if let Ok(res) = solved.solve() {
        println!("{}", res);
    } else {
        println!("err");
    }
    Ok(())
}
