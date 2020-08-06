use super::Die;
use std::collections::HashMap;

pub type DiceProbabilities = HashMap<usize, f32>;

pub fn add(left: &DiceProbabilities, right: &DiceProbabilities) -> DiceProbabilities {
    match (left.is_empty(), right.is_empty()) {
        (false, false) => {
            let mut results = DiceProbabilities::default();

            for (left_value, left_ratio) in left {
                for (right_value, right_ratio) in right {
                    let mul = left_ratio * right_ratio;

                    if let Some(current_ratio) = results.get_mut(&(left_value + right_value)) {
                        *current_ratio += mul;
                    } else {
                        results.insert(left_value + right_value, mul);
                    }
                }
            }

            results
        }
        (true, false) => right.clone(),
        (false, true) => left.clone(),
        (true, true) => DiceProbabilities::default(),
    }
}

pub fn build(die: Die, times: usize) -> DiceProbabilities {
    std::iter::repeat(die.probabilities())
        .take(times)
        .fold(DiceProbabilities::default(), |acc, x| add(&acc, &x))
}
