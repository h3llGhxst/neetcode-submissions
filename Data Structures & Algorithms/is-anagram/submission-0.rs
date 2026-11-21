
use std::{collections::HashMap};

impl Solution {

    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len(){ 
           return false; 
        }

        let mut hash_t : HashMap<u8 , i32> = HashMap::new(); 
        let mut hash_s : HashMap<u8 , i32> = HashMap::new(); 

        for i in 0..s.len() { 
            *hash_s.entry(s.as_bytes()[i]).or_insert(0) += 1 ; 
            *hash_t.entry(t.as_bytes()[i]).or_insert(0) += 1 ; 
        }

        for b in s.as_bytes() { 

            if hash_s.get(b) != hash_t.get(b) { 
                return false
            }

        }

        true 

    }
}
