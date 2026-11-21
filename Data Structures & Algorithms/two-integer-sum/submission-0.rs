
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for (i, value) in nums.into_iter().enumerate() {
            let needed = target - value;

            if let Some(&j) = seen.get(&needed) {
                return vec![j as i32, i as i32];
            }

            seen.insert(value, i);
        }

        Vec::new()
    }
}