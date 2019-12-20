use std::fs;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut v: Vec<String> = fs::read_to_string("input.txt")?.split("\n").map(|item| {item.to_string()}).collect();
    let mut wire_one: Vec<String> = v[0].split(",").map(|item| {item.to_string()}).collect();
    let mut wire_two: Vec<String> = v[1].split(",").map(|item| {item.to_string()}).collect();
//    let mut test = HashSet::new();

    let wire_one_coordinates: HashSet<String> = convert_to_coordinates(&wire_one);
    let wire_two_coordinates: HashSet<String> = convert_to_coordinates(&wire_two);

    let collisions = wire_two_coordinates.intersection(&wire_one_coordinates);

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
*/
fn convert_to_coordinates(directions: &Vec<String>) -> HashSet<String> {
    let mut wire_tracker = [0,0];
    let mut all_touched_coordinates: HashSet<String> = HashSet::new();
    for item in directions.iter() {
        let direction = &item[0..1];

        if direction == "U" {
            let original_coordinate = wire_tracker[1];
            wire_tracker[1] += &item[1..].parse::<i32>().unwrap();

            for i in original_coordinate..wire_tracker[1] {
                let xy: Vec<String> = vec![wire_tracker[0], i].into_iter().map(|item| {item.to_string()}).collect();
                all_touched_coordinates.insert(xy.join());
            }
        } else if direction == "R" {
            let original_coordinate = wire_tracker[1];
            wire_tracker[0] += &item[1..].parse::<i32>().unwrap();
            for i in original_coordinate..wire_tracker[1] {
                let xy: Vec<String> = vec![wire_tracker[0], i].into_iter().map(|item| {item.to_string()}).collect();
                all_touched_coordinates.insert(xy.join());
            }
        } else if direction == "D" {
            let original_coordinate = wire_tracker[1];
            wire_tracker[1] -= &item[1..].parse::<i32>().unwrap();
            for i in original_coordinate..wire_tracker[1] {
                let xy: Vec<String> = vec![wire_tracker[0], i].into_iter().map(|item| {item.to_string()}).collect();
                all_touched_coordinates.insert(xy.join());
            }
        } else if direction == "L" {
            let original_coordinate = wire_tracker[1];
            wire_tracker[0] -= &item[1..].parse::<i32>().unwrap();
            for i in original_coordinate..wire_tracker[1] {
                let xy: Vec<String> = vec![wire_tracker[0], i].into_iter().map(|item| {item.to_string()}).collect();
                all_touched_coordinates.insert(xy.join());
            }
        }
    }

//    println!("{:?}", all_touched_coordinates);
    all_touched_coordinates
}

