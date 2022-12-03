use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
enum Formation {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn main() {
    // Read file into lines
    let file = File::open("/home/piotr/Documents/advent2022/dec2/src/input.txt")
        .expect("Cannot read file!");
    let lines = BufReader::new(file).lines();
    let mut opponent: Formation;
    let mut player: Formation;
    let mut player_second_star: Formation;
    let mut total_score = 0;
    let mut total_score_second_star = 0;

    for line in lines {
        if let Ok(lp) = line {
            opponent = to_formation(&lp.chars().nth(0).unwrap())
                .expect("Wrong value for played formation");
            let second_letter = lp.chars().nth(2).unwrap();
            player = to_formation(&second_letter).expect("Wrong value for played formation");

            total_score += calculate_round_score(&player, &opponent);

            player_second_star = what_to_play(&opponent, &to_outcome(&second_letter).unwrap());
            total_score_second_star += calculate_round_score(&player_second_star, &opponent);
        }
    }

    println!("First star: {}", total_score);

    println!("Second star: {}", total_score_second_star);

    // Second star
}

fn to_formation(letter: &char) -> Result<Formation, &'static str> {
    match letter {
        'C' => Ok(Formation::Scissors),
        'B' => Ok(Formation::Paper),
        'A' => Ok(Formation::Rock),
        'Z' => Ok(Formation::Scissors),
        'Y' => Ok(Formation::Paper),
        'X' => Ok(Formation::Rock),
        _ => Err("No such formation"),
    }
}

fn to_outcome(letter: &char) -> Result<Outcome, &'static str> {
    match letter {
        'Z' => Ok(Outcome::Win),
        'Y' => Ok(Outcome::Draw),
        'X' => Ok(Outcome::Loss),
        _ => Err("No such outcome"),
    }
}

fn formation_value(formation: &Formation) -> i32 {
    match formation {
        Formation::Rock => 1,
        Formation::Paper => 2,
        Formation::Scissors => 3,
    }
}

fn check_if_won(player: &Formation, opponent: &Formation) -> Outcome {
    if player == opponent {
        return Outcome::Draw;
    }
    match player {
        Formation::Paper => match opponent {
            Formation::Rock => Outcome::Win,
            _ => Outcome::Loss,
        },
        Formation::Rock => match opponent {
            Formation::Scissors => Outcome::Win,
            _ => Outcome::Loss,
        },
        Formation::Scissors => match opponent {
            Formation::Paper => Outcome::Win,
            _ => Outcome::Loss,
        },
    }
}

fn outcome_multiplier(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn what_to_play(opponent: &Formation, desired_outcome: &Outcome) -> Formation {
    if *desired_outcome == Outcome::Draw {
        return opponent.clone();
    }
    match opponent {
        Formation::Paper => match desired_outcome {
            Outcome::Win => Formation::Scissors,
            _ => Formation::Rock,
        },
        Formation::Rock => match desired_outcome {
            Outcome::Win => Formation::Paper,
            _ => Formation::Scissors,
        },
        Formation::Scissors => match desired_outcome {
            Outcome::Win => Formation::Rock,
            _ => Formation::Paper,
        },
    }
}

fn calculate_round_score(player: &Formation, opponent: &Formation) -> i32 {
    formation_value(&player) + outcome_multiplier(&check_if_won(&player, &opponent))
}
