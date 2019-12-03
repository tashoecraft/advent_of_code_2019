use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let test = contents.split_whitespace();
    let output = test.fold(0.0, |mut acc, x| {
        acc += calc_total(mass_calc(x.parse().unwrap()));
        acc
     } );

    println!("{}", output); 
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
