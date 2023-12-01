use std::fs;

fn main() {
    let contents = fs::read_to_string("config.txt").expect("Expected input file here");

    let r: u32 = contents
        .lines()
        .map(|s| -> u32 {
            let mut i = s.chars().filter(|c| c.is_ascii_digit());
            let f = i
                .next()
                .expect("Expected at least one number on every line");
            let l = i.last().unwrap_or(f);
            (f.to_digit(10).unwrap()) * 10 + (l.to_digit(10).unwrap())
        })
        .sum();

    println!("{r}");
}
