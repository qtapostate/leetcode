use std::collections::{HashSet};

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut substrings = 1;
        let mut seen = HashSet::<u8>::new();

        for byte in s.into_bytes().into_iter() {
            if !seen.insert(byte) {
                substrings += 1;
                seen.clear();
            }

            seen.insert(byte);
        }

        substrings
    }
}