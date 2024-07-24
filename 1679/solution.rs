use std::cmp::Ordering;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut filtered = nums.into_iter().filter(|&v| v < k).collect::<Vec<_>>();
        filtered.sort();

        if filtered.len() < 2 {
            return 0;
        }

        let mut i = 0;
        let mut j = filtered.len() - 1;
        let mut count = 0;

        while i < j {
            match (filtered[i] + filtered[j]).cmp(&k) {
                Ordering::Equal => {
                    count += 1;
                    i += 1;
                    j -= 1;
                },
                Ordering::Less => {
                    i += 1;
                },
                Ordering::Greater => {
                    j -= 1;
                }
            }
        }

        return count;
    }
}