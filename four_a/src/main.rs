use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut found_passwords = vec![];
    for password in 359282..820401 {
        let password_found = check_password(password);
        if password_found {
            found_passwords.push(password);
        }
    }

    println!("{:?}", found_passwords.len());
    Ok(())
}

fn check_password(pass: i32) -> bool {
    // check that number does not have digits on the left side that are greater than the right side
    // insure atleast one set of the same
    let mut previous_digit = 0;
    let mut duplicate_train = vec![];
    let mut duplicate_found = false;
    for i in pass.to_string().chars() {
        let num = i.to_digit(10).unwrap();
        if previous_digit > num {
            return false;
        }
        if previous_digit == num {
            duplicate_found = true;
        }

        duplicate_train.push(num);
        
        previous_digit = num;
    }
    if duplicate_found {
        let find_double: bool = duplicate_train.iter()
            .fold(HashMap::new(), |mut accum, x| {
                if accum.contains_key(x) {
                    *accum.get_mut(&x).unwrap() += 1;
                } else {
                    accum.insert(x, 1);
                }
                accum
            })
            .iter()
            .fold(false, |mut found, (key, value)| {
                if *value == 2 {
                    found = true;
                }
                found
            });
        return find_double
    }

    
    return false
}
