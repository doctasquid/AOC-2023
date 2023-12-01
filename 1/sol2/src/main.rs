use std::fs;
use std::cmp::min;

fn main() {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let contents = fs::read_to_string("config.txt").expect("Expected input file here");

    let mut sum = 0;

    for line in contents.lines() {
        let mut first = 11;
        let mut last = 0;
        let char_n: usize = line.len();
        let mut chars = line.chars();
        for i in 0..char_n {
            let cc = chars.next().unwrap();
            if cc.is_ascii_digit() {
                let n = cc.to_digit(10).unwrap();
                last = n;
                if first == 11 {
                    first = n;
                }
                continue;
            }
            for j in 0..=9 {
                let end = min(i + digits[j].len(),char_n);
                if &line[i..end] == digits[j] {
                    last = j as u32;
                    if first == 11 {
                        first = j as u32;
                    }
                }
            }
        }
        sum += first * 10 + last;
    }

    println!("{sum}");
}
