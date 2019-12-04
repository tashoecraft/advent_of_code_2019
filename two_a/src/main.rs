use std::fs;
use std::process;
fn main() -> std::io::Result<()> {
    let mut v: Vec<_> = fs::read_to_string("input.txt")?.split(",")
        .map(|item| {
            let mut s = item.to_string();
            if s.ends_with('\n') {
                s.pop();
            }
            s.parse::<i32>().unwrap()
        })
        .collect();
    let mut copy_v: Vec<_> = v.iter().cloned().collect();
    let mut index: i32 = 0;
    let mut current_opcode: i32 = copy_v[index];

    while current_opcode % 99 != 0 {
        if current_opcode % 2 == 0 {
            let i:i32 = copy_v.get_mut(index+3).unwrap();
            let first:i32 = copy_v.get_mut(index+1);
            let second:i32 = copy_v.get_mut(index+3);
            copy_v[i] = copy_v.get_mut(first) * copy_v.get_mut(second);
            index += 4;
            current_opcode = copy_v[index];
        } else if current_opcode % 1 == 0 {
            let j:i32 = copy_v.get_mut(index+3);
            let first:i32 = copy_v.get_mut(index+1);
            let second:i32 = copy_v.get_mut(index+3);
            copy_v[index] = copy_v.get_mut(first) + copy_v.get_mut(second);
            index += 4;
            current_opcode = copy_v.get_mut(index);
        } else {
            println!("PANIC");
            process::exit(1);
        }
    }
    println!("{:?}", v);

    Ok(())
}
