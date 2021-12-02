#![feature(test)]

pub mod aoc2020;
pub mod aoc2021;
pub mod utils {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    /// read_lines returns a BufReader Iterator of a file wrapped in a Result
    ///
    /// The output is wrapped in a Result to allow matching on errors
    /// Returns an Iterator to the Reader of the lines of the file.
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
