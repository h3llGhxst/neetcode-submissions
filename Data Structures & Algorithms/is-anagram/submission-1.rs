impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut hash_s = HashMap::new();
        let mut hash_t = HashMap::new();

        for (a, b) in s.bytes().zip(t.bytes()) {
            *hash_s.entry(a).or_insert(0) += 1;
            *hash_t.entry(b).or_insert(0) += 1;
        }

        hash_s == hash_t
    }
}