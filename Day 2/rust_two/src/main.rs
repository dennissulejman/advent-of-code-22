use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const LOSS_PTS: i32 = 0;
const DRAW_PTS: i32 = 3;
const WIN_PTS: i32 = 6;

const ROCK_PTS: i32 = 1;
const PAPER_PTS: i32 = 2;
const SCISSORS_PTS: i32 = 3;

const PLAYER_ROCK: char = 'X';
const PLAYER_PAPER: char = 'Y';
const PLAYER_SCISSORS: char = 'Z';

const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B';
const OPPONENT_SCISSORS: char = 'C';

const ROCK: &str = "Rock";
const PAPER: &str = "Paper";
const SCISSORS: &str = "Scissors";

fn main() {
    // Part 1
    let mut player_score = 0;
    // Part 2
    let mut player_optimal_score = 0;

    let filename = "../Input.txt";

    if let Ok(lines) = read_lines(filename) {
        for line_result in lines {
            if let Ok(line) = line_result {
                let line_chars: Vec<char> = line.chars().collect();

                let oppponent_shape_name = get_shape_name(line_chars[0]);
                let player_shape_name = get_shape_name(line_chars[2]);

                let opponent_shape = init_shape(&oppponent_shape_name);
                let player_shape = init_shape(&player_shape_name);
                let player_optimal_shape =
                    init_optimal_player_shape(&player_shape, &opponent_shape);

                player_score += player_shape.points;
                player_score += get_player_round_result(&player_shape, &opponent_shape);

                player_optimal_score += player_optimal_shape.points;
                player_optimal_score +=
                    get_player_round_result(&player_optimal_shape, &opponent_shape);
            }
        }
    }

    println!("{player_score}");
    println!("{player_optimal_score}");
}

fn get_player_round_result(player_shape: &Shape, opponent_shape: &Shape) -> i32 {
    if player_shape.name == opponent_shape.name {
        return DRAW_PTS;
    } else if player_shape.name == opponent_shape.weaker_than {
        return WIN_PTS;
    } else {
        return LOSS_PTS;
    }
}

fn get_shape_name(shape: char) -> String {
    return match shape {
        PLAYER_ROCK | OPPONENT_ROCK => String::from(ROCK),
        PLAYER_PAPER | OPPONENT_PAPER => String::from(PAPER),
        PLAYER_SCISSORS | OPPONENT_SCISSORS => String::from(SCISSORS),
        _ => unreachable!(),
    };
}

fn init_shape(shape_name: &String) -> Shape {
    return match shape_name.as_str() {
        ROCK => Shape {
            name: String::from(ROCK),
            stronger_than: String::from(SCISSORS),
            weaker_than: String::from(PAPER),
            points: ROCK_PTS,
        },
        PAPER => Shape {
            name: String::from(PAPER),
            stronger_than: String::from(ROCK),
            weaker_than: String::from(SCISSORS),
            points: PAPER_PTS,
        },
        SCISSORS => Shape {
            name: String::from(SCISSORS),
            stronger_than: String::from(PAPER),
            weaker_than: String::from(ROCK),
            points: SCISSORS_PTS,
        },
        _ => unreachable!(),
    };
}

fn init_optimal_player_shape(player_shape: &Shape, opponent_shape: &Shape) -> Shape {
    return match player_shape.name.as_str() {
        ROCK => init_shape(&opponent_shape.stronger_than),
        PAPER => init_shape(&opponent_shape.name),
        SCISSORS => init_shape(&opponent_shape.weaker_than),
        _ => unreachable!(),
    };
}

struct Shape {
    name: String,
    stronger_than: String,
    weaker_than: String,
    points: i32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
