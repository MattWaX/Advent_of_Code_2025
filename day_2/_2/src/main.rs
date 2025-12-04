use std::env;
use std::fs;

fn main() {
    let ids = fs::read_to_string(env::args().nth(1).unwrap()).expect("Expected a valid file path");
    let mut sum = 0;

    for range in ids.split(",") {
        let (bot, up) = range.split_once("-").unwrap();
        let (bot, up) = (bot.trim_end_matches("\n"), up.trim_end_matches("\n"));

        for i in bot.parse::<u64>().unwrap()..=up.parse::<u64>().unwrap() {
            let n = i.to_string();
            let len = n.len();

            let mut is_equal: bool = true;
            let mut prev: &str;

            for j in 2..=len {
                if len % j != 0 {
                    continue;
                }

                is_equal = true;

                let div = len / j;

                prev = &n[0..div];

                for k in 1..(len / div) {
                    if prev != &n[k * div..k * div + div] {
                        is_equal = false;
                    }
                }
                if is_equal == true {
                    println!("{i} is invalid");
                    sum += i;
                    break;
                }
            }
        }
    }
    println!("The sum of invalid ids is {sum}");
}
