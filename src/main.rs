use game_curve_probabilities::parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_die = parser::parse(&args[1]);

    match parsed_die {
        Ok(("", dice)) => {
            let mut probs: Vec<(usize, f32)> = dice.probabilities().into_iter().collect();
            probs.sort_by(|x, y| x.0.cmp(&y.0));
            for (value, prob) in probs.into_iter().collect::<Vec<(usize, f32)>>() {
                println!("{:6}: {:.4}%", value, prob * 100.0);
            }
        }
        Ok((leftovers, dice)) => {
            eprintln!("Unable to parse some input: '{}'", leftovers);

            let mut probs: Vec<(usize, f32)> = dice.probabilities().into_iter().collect();
            probs.sort_by(|x, y| x.0.cmp(&y.0));
            for (value, prob) in probs.into_iter().collect::<Vec<(usize, f32)>>() {
                println!("{:6}: {:.4}%", value, prob * 100.0);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}
