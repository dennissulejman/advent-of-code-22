use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_elf_calories_carried = 0;
    let mut calories_carried = vec![];

    let filename = "../Input.txt";

    if let Ok(lines) = read_lines(filename) {
        for line_result in lines {
            if let Ok(line) = line_result {
                if let Ok(num) = line.parse::<i32>() {
                    current_elf_calories_carried += num;
                }
                if line.is_empty() {
                    calories_carried.push(current_elf_calories_carried);
                    current_elf_calories_carried = 0;
                }
            }
        }
    }

    // Make sure to include the last elf
    calories_carried.push(current_elf_calories_carried);

    calories_carried.sort_by(|a, b| b.cmp(a));
    calories_carried.truncate(3);
    let top_one_calories_carried = calories_carried[0];
    let top_three_calories_carried_sum = calories_carried.iter().sum::<i32>();
    println!("{top_one_calories_carried}");
    println!("{top_three_calories_carried_sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
