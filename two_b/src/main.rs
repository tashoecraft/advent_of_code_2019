use std::fs;

fn main() -> std::io::Result<()> {
    let mut v: Vec<i32> = fs::read_to_string("input.txt")?.split(",")
        .map(|item| {
            let mut s = item.to_string();
            if s.ends_with('\n') {
                s.pop();
            }
            s.parse::<i32>().unwrap()
        }).collect();
    let mut copy_v: Vec<i32> = v.iter().cloned().collect();

    let mut x = 0;
    let mut y = 0;
    let result = split_caller(&mut x, &mut y, copy_v);

    println!("{:?}", result);

    Ok(())
}

/**
 * function that returns 100 * [1] [2] from executing computer with modified [1][2]
 *
algo split problem in half
run down each branch incrementing one half by one
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
fn split_caller(first_number: &mut i32, second_number: &mut i32, computer: Vec<i32>) -> Vec<i32> {
    if *first_number == 100 && *second_number == 100 {
        return vec![0, 0];
    }

    let output_first = run_one_side(first_number, second_number, &String::from("one"), &computer);
    let output_second = run_one_side(first_number, second_number, &String::from("second"), &computer);
    let total_first = calculate_score(&output_first[1], &output_first[2]);
    let total_second = calculate_score(&output_second[1], &output_second[2]);


    if output_first[0] == 19690720 {
        return vec![total_first]
    } else if output_second[0] == 19690720 {
        return vec![total_first]
    } else {
        let mut upped_first_number = *first_number + 1;
        let mut upped_second_number = *second_number + 1;
        return split_caller(&mut upped_first_number, &mut upped_second_number, computer);
    }
}

fn calculate_score(first: &i32, second: &i32) -> i32 {
    return 100 * first + second;
}

fn run_one_side(first_number: &mut i32, second_number: &mut i32, side_to_run: &str, computer: &Vec<i32>) -> Vec<i32> {
    let mut copy_of_computer = clone_and_modify(first_number, second_number, computer);
    let output: &mut Vec<i32> = execute_computer(&mut copy_of_computer);
     if output[0] == 19690720 {
         return output.clone()
     }

     if side_to_run == "one" {
          if *first_number == 99 {
              return vec![0,0,0];
          }
         let mut updated_first_number = *first_number + 1;
      return run_one_side(&mut updated_first_number, second_number, &side_to_run, &computer)
     } else {
         if *second_number == 99 {
              return vec![0,0,0];
         }
         let mut updated_second_number = *second_number + 1;

         return run_one_side(first_number, &mut updated_second_number, side_to_run, &computer)
     }
}

fn clone_and_modify(first_number: &mut i32, second_number: &mut i32, computer: &Vec<i32>) -> Vec<i32> {
    let mut cloned = computer.clone();
    cloned[1] = *first_number;
    cloned[2] = *second_number;
    cloned
}

fn execute_computer(computer_code: &mut Vec<i32>) -> &mut Vec<i32> {
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

    computer_code
}
