impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        nums
            .windows(k as usize)
            .map(|window| window.into_iter().sum())
            .max()
            .unwrap_or(0) as f64 / (k as f64)
    }
}