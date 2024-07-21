impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut count_zeroes = nums.iter().filter(|&v| *v == 0i32).count();
        nums.retain(|&v| v != 0);
        nums.append(&mut vec![0i32; count_zeroes]);
    }
}