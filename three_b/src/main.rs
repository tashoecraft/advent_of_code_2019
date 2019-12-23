use std::fs;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let v: Vec<String> = fs::read_to_string("input.txt")?.split("\n").map(|item| {item.to_string()}).collect();
    let wire_one: Vec<String> = v[0].split(",").map(|item| {item.to_string()}).collect();
    let wire_two: Vec<String> = v[1].split(",").map(|item| {item.to_string()}).collect();
//    let mut test = HashSet::new();

    let wire_one_coordinates: HashMap<String, i32> = convert_to_coordinates(&wire_one);
    let wire_two_coordinates: HashMap<String, i32> = convert_to_coordinates(&wire_two);

    let collisions: i32 = wire_two_coordinates.keys().fold(100000, |acc, x| {
        if wire_one_coordinates.contains_key(x) {
            let total = wire_one_coordinates.get(x).unwrap().abs() + wire_two_coordinates.get(x).unwrap().abs();
            if acc > total {
                return total;
            }
        }

        acc
    });
    println!("{:?}", collisions);
    Ok(())

}

/*
* Construct a grid of x and y coordinates
* starting at 0,0
* R8,U5,L5,D3
* U7,R6,D4,L4
* 8,0 -> 8,5 -> 3,5 -> 3,2
* 0,7 -> 5,7 -> 5,3 -> 1,3
* Record is hashmap? each coordinate that is hit in first wire
* Go through series for second wire, check if value is in hash of first
* if so mark it.
* Loop through hits and find one with value closet to 0,0
* 
* r8
*
*/
fn convert_to_coordinates(directions: &Vec<String>) -> HashMap<String, i32> {
    let mut wire_tracker = [0,0];
    let mut all_touched_coordinates: HashMap<String, i32> = HashMap::new();
    let mut steps = 1;
    
    for item in directions.iter() {
        let test = item.trim();

        let direction = &test[0..1];
        if direction == "U" {

            let original_coordinate = wire_tracker[1];
            wire_tracker[1] += &test[1..].parse::<i32>().unwrap();
            for i in original_coordinate+1..wire_tracker[1]+1 {
                let xy: Vec<String> = vec![wire_tracker[0], i].into_iter().map(|item| {item.to_string()}).collect();
                all_touched_coordinates.insert(xy.join(","), steps);
                steps += 1;
            }

        } else if direction == "R" {
            let original_coordinate = wire_tracker[0];
            wire_tracker[0] += &test[1..].parse::<i32>().unwrap();
            for i in original_coordinate+1..wire_tracker[0]+1 {
                let xy: Vec<String> = vec![i, wire_tracker[1]].into_iter().map(|item| {item.to_string()}).collect();
                all_touched_coordinates.insert(xy.join(","), steps);
                steps += 1;
            }

        } else if direction == "D" {
            let original_coordinate = wire_tracker[1];
            wire_tracker[1] -= &test[1..].parse::<i32>().unwrap();
            for i in (wire_tracker[1]..original_coordinate).rev() {
                let xy: Vec<String> = vec![wire_tracker[0], i].into_iter().map(|item| {item.to_string()}).collect();
                all_touched_coordinates.insert(xy.join(","), steps);
                steps += 1;
            }
        } else if direction == "L" {
            let original_coordinate = wire_tracker[0];
            wire_tracker[0] -= &test[1..].parse::<i32>().unwrap();
            for i in (wire_tracker[0]..original_coordinate).rev() {
                let xy: Vec<String> = vec![i, wire_tracker[1]].into_iter().map(|item| {item.to_string()}).collect();
                all_touched_coordinates.insert(xy.join(","), steps);
                steps += 1;
            }
        }
    }

//    println!("{:?}", all_touched_coordinates);
    all_touched_coordinates
}

// fn 

fn find_min<'a, I>(coordinates: I) -> i32
where
    I: Iterator<Item = &'a String>
{
    let mut min = 100000;

    for coordinate in coordinates {
        println!("{:?}", coordinate);

        let total = coordinate.split(",").fold(0, |mut acc: i32, x: &str| {
            acc += x.parse::<i32>().unwrap().abs();
            acc
        }); 


        if total < min {
            min = total;
        }
    }
    min
}

