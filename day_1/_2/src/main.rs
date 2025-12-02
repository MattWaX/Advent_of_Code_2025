use std::env;
use std::fs;

// Prints each argument on a separate line
fn main() {
    let mut dial: i64 = 50;
    let mut password: u64 = 0;
    let doc = fs::read_to_string(env::args().nth(1).unwrap()).expect("File expected");

    for line in doc.lines() {
        let shift: u32 = line[1..]
            .parse::<u32>()
            .expect("Must be a number after the first char");

        match line.chars().nth(0).unwrap() {
            'R' => {
                for _i in 0..shift {
                    dial += 1;
                    if dial > 99 {
                        dial = 0;
                        password += 1;
                    }
                }
            }
            'L' => {
                for _i in 0..shift {
                    dial -= 1;
                    if dial == 0 {
                        password += 1
                    }
                    if dial < 0 {
                        dial = 99;
                    }
                }
            }
            _ => (),
        }
    }
    println!("The password is: {password}");
}
