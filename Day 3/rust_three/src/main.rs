use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let mut items_priority_sums = 0;

    let filename = "../Input.txt";

    if let Ok(lines) = read_lines(filename) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let full_len = line.len();
                let half_len = full_len / 2;

                let first_half = &line[0..half_len];
                let last_half = &line[half_len..full_len];

                for (_, c) in first_half.chars().enumerate() {
                    if last_half.contains(c) {
                        // Add one at the end because the priority starts at 1
                        let priority = ALPHABET.find(c).expect("Not found!") + 1;
                        items_priority_sums += priority;

                        // Only get the priority of the first match
                        break;
                    }
                }
            }
        }
    }

    println!("{items_priority_sums}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
