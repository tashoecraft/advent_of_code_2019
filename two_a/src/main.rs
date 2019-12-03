use std::fs;

fn main() -> std::io::Result<()> {
//    let mut contents: String = fs::read_to_string("input.txt")?;

    let mut v: Vec<i32> = fs::read_to_string("input.txt")?.split(",")
                            .collect()
                            .map::<str>(|item| {
                                if item.ends_with('\n') {
                                    item.pop();
                                }

                                item.parse::<i32>()
                            });
    println!("{:?}", v);

    Ok(())
}
