use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const PATH_CONCEPT: &str = "all/concept/concept.txt";
//const PATH_CONCEPT: &str = "../../all/concept/concept.txt";

pub fn read_concept() -> Result<(), Error> {
    let path = PATH_CONCEPT;

    let f_input = File::open(path)?;
    let buffered = BufReader::new(f_input);

    for line in buffered.lines() {
        println!("\t{}", line?);
    }

    println!();

    Ok(())
}