use super::probabilities;
use super::{DiceProbabilities, Die};
use std::default::Default;

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
            .iter()
            .fold(DiceProbabilities::default(), |acc, x| {
                probabilities::add(&acc, &x)
            })
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
}
