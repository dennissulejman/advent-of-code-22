use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut fully_contained_assignment_pairs = 0;

    let filename = "../Input.txt";

    if let Ok(lines) = read_lines(filename) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let assignment_pairs = line.split(',').collect::<Vec<&str>>();

                let first_assignment = assignment_pairs[0].split('-').collect::<Vec<&str>>();
                let second_assignment = assignment_pairs[1].split('-').collect::<Vec<&str>>();

                let first_assignment_min = first_assignment[0].parse::<i32>().unwrap();
                let first_assignment_max = first_assignment[1].parse::<i32>().unwrap();

                let second_assignment_min = second_assignment[0].parse::<i32>().unwrap();
                let second_assignment_max = second_assignment[1].parse::<i32>().unwrap();

                if (first_assignment_min <= second_assignment_min
                    && first_assignment_max >= second_assignment_max)
                    || (second_assignment_min <= first_assignment_min
                        && second_assignment_max >= first_assignment_max)
                {
                    fully_contained_assignment_pairs += 1;
                }
            }
        }
    }
    println!("{fully_contained_assignment_pairs}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
