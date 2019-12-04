use std::fs;
fn main() -> std::io::Result<()> {
    let mut v: Vec<i32> = fs::read_to_string("input.txt")?.split(",")
        .map(|item| {
            let mut s = item.to_string();
            if s.ends_with('\n') {
                s.pop();
            }
            s.parse::<i32>().unwrap()
        })
        .collect();
    let mut copy_v: Vec<i32> = v.iter().cloned().collect();
    let mut index: i32 = 0;
    let mut current_opcode: i32 = copy_v[index as usize];

    while current_opcode % 99 != 0 {
        if current_opcode % 2 == 0 {
            let i: i32 = copy_v[(index + 3) as usize];
            let first: i32 = copy_v[(index + 1) as usize];
            let second: i32 = copy_v[(index + 2) as usize];
            copy_v.push(copy_v[first as usize] * copy_v[second as usize]);
            copy_v.swap_remove(i as usize);
            index += 4;
            current_opcode = copy_v[index as usize];
        } else {
            let j: i32 = copy_v[(index + 3) as usize];
            let first: i32 = copy_v[(index + 1) as usize];
            let second: i32 = copy_v[(index + 2) as usize];
            copy_v.push(copy_v[first as usize] + copy_v[second as usize]);
            copy_v.swap_remove(j as usize);
            index += 4;
            current_opcode = copy_v[index as usize];
        }
    }

    println!("{:?}", copy_v);

    Ok(())
}
