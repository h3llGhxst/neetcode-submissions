use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();

        for v in nums {
            if !seen.insert(v) {
                return true;
            }
        }

        false
    }
}