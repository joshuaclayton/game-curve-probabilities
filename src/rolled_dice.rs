use super::probabilities;
use super::{DiceProbabilities, Die};
use rand::distributions::Uniform;
use rand::prelude::*;
use rayon::prelude::*;
use std::default::Default;

pub struct RollOutcome {
    pub rolls: Vec<(Die, usize)>,
}

impl RollOutcome {
    pub fn total(&self) -> usize {
        self.rolls.iter().map(|v| v.1).sum()
    }
}

#[derive(Debug)]
pub struct RolledDice {
    pub d4: usize,
    pub d6: usize,
    pub d8: usize,
    pub d10: usize,
    pub d12: usize,
    pub d20: usize,
}

impl Default for RolledDice {
    fn default() -> Self {
        RolledDice {
            d4: 0,
            d6: 0,
            d8: 0,
            d10: 0,
            d12: 0,
            d20: 0,
        }
    }
}

impl RolledDice {
    pub fn probabilities(&self) -> DiceProbabilities {
        let d4s = probabilities::build(Die::D4, self.d4);
        let d6s = probabilities::build(Die::D6, self.d6);
        let d8s = probabilities::build(Die::D8, self.d8);
        let d10s = probabilities::build(Die::D10, self.d10);
        let d12s = probabilities::build(Die::D12, self.d12);
        let d20s = probabilities::build(Die::D20, self.d20);

        vec![d4s, d6s, d8s, d10s, d12s, d20s]
            .into_iter()
            .par_bridge()
            .reduce(
                || DiceProbabilities::default(),
                |acc, x| probabilities::add(&acc, &x),
            )
    }

    pub fn add_dice(&mut self, count: &usize, die: &Die) {
        match die {
            Die::D4 => self.d4 += count,
            Die::D6 => self.d6 += count,
            Die::D8 => self.d8 += count,
            Die::D10 => self.d10 += count,
            Die::D12 => self.d12 += count,
            Die::D20 => self.d20 += count,
        }
    }

    pub fn roll(&self, mut rng: ThreadRng) -> RollOutcome {
        let mut rolls = vec![];

        for die in vec![
            std::iter::repeat(Die::D4).take(self.d4),
            std::iter::repeat(Die::D6).take(self.d6),
            std::iter::repeat(Die::D8).take(self.d8),
            std::iter::repeat(Die::D10).take(self.d10),
            std::iter::repeat(Die::D12).take(self.d12),
            std::iter::repeat(Die::D20).take(self.d20),
        ]
        .into_iter()
        .flatten()
        {
            let distribution = Uniform::new_inclusive(die.minimum(), die.maximum());
            rolls.push((die, distribution.sample(&mut rng)));
        }

        RollOutcome { rolls }
    }
}
