

use std::collections::HashMap;

impl Solution {

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();

        let mut counts: HashMap<i32, usize> = HashMap::new();
        for x in nums {
            *counts.entry(x).or_insert(0) += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![vec![]; n + 1];
        for (num, freq) in counts {
            buckets[freq].push(num);
        }

        buckets.into_iter().rev().flatten().take(k as usize).collect()
    }
}