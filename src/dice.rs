use super::DiceProbabilities;

#[derive(Clone, Debug)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Die {
    pub fn minimum(&self) -> usize {
        1
    }

    pub fn maximum(&self) -> usize {
        match self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
        }
    }

    pub fn probabilities(&self) -> DiceProbabilities {
        (self.minimum()..=self.maximum())
            .map(|v| (v, 1.0 / (self.maximum() as f32)))
            .collect()
    }
}
