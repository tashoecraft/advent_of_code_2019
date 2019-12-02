use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut test = contents.split_whitespace();
    let mut sum = 0;
    loop {
        match test.next() {
            Some(x) => {
                sum += massCalc(x);
            },
            None => { break }
        }
    }   
    println!("{}", sum); 
    Ok(())
}


fn massCalc(numAsStr: &str) -> i32 {
    numAsStr.parse::<i32>().unwrap() / 3 - 2
}
