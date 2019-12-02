use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut test = contents.split_whitespace();
    let mut sum = 0.0;
    loop {
        match test.next() {
            Some(x) => {
                sum += calc_total(mass_calc(x.parse().unwrap()));                
            },
            None => { break }
        }
    }   
    println!("{}", sum); 
    Ok(())
}

fn calc_total(input: f32) -> f32 {
    let mut sum = input;
    if input > 0.0 {
        sum += calc_total(mass_calc(input));
        sum
    } else {
        0.0
    }

}


fn mass_calc(num: f32) -> f32 {
    ((num / 3.0) - 2.0).floor()
}
