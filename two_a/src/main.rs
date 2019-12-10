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

    
    // recrusice solution
    /**
     * function that returns 100 * [1] [2] from executing computer with modified [1][2]
     * 
    algo split problem in half
    run down each branch increating one half by one
    increase both by 1
    repeat

    0 0 one
        1 0
            2 0
                3 0

    0 0 two
        0 1
            0 2
                0  3

    1 1 one
        2 1
            3 1
                4   1
    1 1 two
        1 2
            1 3
                1 4

    2 2 one
        3 2
            4 2
                4 2
    2 2 two 
        2 3
            2 4
                2 5 

    * 
    * 
    * function to call both 
    * (firstNum, secondNum, numToHit)
    *
    * if val !== numToHit
    *   num1 = firstNum ++
    *   num2 = secondNum ++
    *   out1 = funToRun1(num1, num2, one)
    *   ou2 = funToRunt2(num1, num2, two) 
    * if out1
    *   out1
    * if out2
    *   out2
    *
    * function to run one and incrementing one side
    *  function ([1], [2], valToIncrease (one or two))
    *    one = [1]
    *    two = [2]
    *    out = execute_comp(1, 2)
    *    if out == numToHit
    *       1 2
    *    else 
    *      valToIncrease 1
    *            one ++
    *        else 
    *            two ++
    *
    *    func(one, two, valToIncrease)
    * 
    */
    execute_computer(&mut copy_v);

    println!("{:?}", copy_v);

    Ok(())
}

fn split_caller(fistNumber: &i32, secondNumber: &i32) -> Vec<i32> {
    if firstNumber == 99 && secondNumber == 99 {
        return vec![0, 0];
    }

    // incFirst = firstNumber ++
    // incSecond = secondNumber ++
    let outputFirst = run_one_side(incFirst, incSecond, 'one');
    let outputSecond = run_one_side(incFirst, incSecond, 'second');
    
    if outputFirst ==
}

fn calculate_score(first: &usize, second: &usize): usize {
    return 100 * first + second;
}

fn run_one_side(firstNumber: &i32, secondNumber: &i32, sideToRun: &str) => Vec<i32> {
    // output = execute_computer(clone_and_modify(firstNumber, secondNumber))

    // if output == numToHit
    //  return vec![firstNumber, secondNumber]

    // if sideToRun == 'one'
    //  if firstNumber ++ > 99
    //      return vec![0,0];
    //  return run_one_side(firstNumber ++, second, sideToRun)
    // else 
    // if secondNUmber ++ > 99
    //      return vec![0,0]
    //  return run_one_side(firstNumber ++, second, sideToRun)
}

fn clone_and_modify(firstNumber: &i132, secondNumber: &i32): Vec<i32> {
    // cloned = clone(copy_v)
    // cloned[1] = firstNumber
    // cloned[2] = second number
    // return cloned
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
