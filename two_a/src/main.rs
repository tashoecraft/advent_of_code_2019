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

    execute_computer(&mut copy_v);

    println!("{:?}", copy_v);

    Ok(())
}

fn execute_computer(computer_code: &mut Vec<i32>) {
    let mut index: i32 = 0;
    let mut current_opcode: i32 = computer_code[index as usize];
    while current_opcode % 99 != 0 {
        if current_opcode % 2 == 0 {
            let i: i32 = computer_code[(index + 3) as usize];
            let first: i32 = computer_code[(index + 1) as usize];
            let second: i32 = computer_code[(index + 2) as usize];
            computer_code.push(computer_code[first as usize] * computer_code[second as usize]);
            computer_code.swap_remove(i as usize);
            index += 4;
            current_opcode = computer_code[index as usize];
        } else {
            let j: i32 = computer_code[(index + 3) as usize];
            let first: i32 = computer_code[(index + 1) as usize];
            let second: i32 = computer_code[(index + 2) as usize];
            computer_code.push(computer_code[first as usize] + computer_code[second as usize]);
            computer_code.swap_remove(j as usize);
            index += 4;
            current_opcode = computer_code[index as usize];
        }
    }

//    copy_v
}
