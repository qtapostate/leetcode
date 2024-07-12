impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut a: i32 = i32::MAX;
        let mut b: i32 = i32::MAX;
        let mut c: i32 = i32::MAX;

        let mut i: usize = 0usize;

        return loop {
            if i == nums.len() {
                break false;
            }

            if a >= nums[i] {
                a = nums[i];
            }
            else if b >= nums[i] {
                b = nums[i];
            }
            else if c >= nums[i] {
                break true;
            }

            i += 1;
        };
    }
}