use std::env;
use std::fs;

fn main() {
    let ids = fs::read_to_string(env::args().nth(1).unwrap()).expect("Expected a valid file path");
    let mut sum = 0;

    for range in ids.split(",") {
        // println!("{range}");
        let (bot, up) = range.split_once("-").unwrap();
        let (bot, up) = (bot.trim_end_matches("\n"), up.trim_end_matches("\n"));

        for i in bot.parse::<u64>().unwrap()..=up.parse::<u64>().unwrap() {
            let n = i.to_string();
            let len = n.len() - 1;

            if n[0..=(len / 2)] == n[(len / 2) + 1..] {
                sum += i;
                println!("{i}");
            }
        }
    }
    println!("The sum of invalid ids is {sum}");
}
