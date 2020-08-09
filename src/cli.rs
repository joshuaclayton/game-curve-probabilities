use super::{parser, RolledDice};
use rand::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "A CLI to caluclate random dice rolls",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
struct Config {
    /// Dice format
    dice_counts: String,

    /// Display each die roll
    #[structopt(short)]
    display_each_die_roll: bool,

    /// Display value probabilities
    #[structopt(short)]
    probabilities: bool,
}

pub fn run() {
    let config = Config::from_args();

    match parser::parse(&config.dice_counts) {
        Ok((input, dice)) => {
            display_results(&config, input, &dice);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}

fn display_results(config: &Config, unparsed_input: &str, dice: &RolledDice) {
    match unparsed_input {
        "" => {}
        leftovers => eprintln!("Unable to parse some input: '{}'", leftovers),
    }

    let roll = dice.roll(thread_rng());

    println!("{}", roll.total());

    if config.display_each_die_roll {
        println!("rolled: {:?}", roll.rolls);
    }

    if config.probabilities {
        display_probabilities(&dice);
    }
}

fn display_probabilities(dice: &RolledDice) {
    let mut probs: Vec<(usize, f32)> = dice.probabilities().into_iter().collect();
    probs.sort_by(|x, y| x.0.cmp(&y.0));
    for (value, prob) in probs {
        println!("{:6}: {:.4}%", value, prob * 100.0);
    }
}
