use std::env;
use std::fs;

// Prints each argument on a separate line
fn main() {
    let mut dial: i64 = 50;
    let mut password: usize = 0;
    let doc = fs::read_to_string(env::args().nth(1).unwrap()).expect("File expected");

    for line in doc.lines() {
        let mut shift: i64 = line[1..]
            .parse::<i64>()
            .expect("Must be a number after the first char");

        if line.chars().nth(0).unwrap() == 'L' {
            shift = -shift;
        }

        dial += shift;

        while dial > 99 {
            dial -= 100;
        }
        while dial < 0 {
            dial += 100;
        }

        if dial == 0 {
            password += 1;
        }
    }
    println!("The password is: {password}");
}
