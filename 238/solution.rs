impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut products = vec![1; nums.len()];
        let mut left = vec![1; nums.len()];
        let mut right = vec![1; nums.len()];

        let mut i = 1;
        loop {
            if i >= nums.len() {
                break;
            }

            left[i] = left[i - 1] * nums[i - 1];

            i += 1;
        }

        i = nums.len() - 2;
        loop {
            right[i] = right[i + 1] * nums[i + 1];

            if i == 0 {
                break;
            }

            i -= 1;
        }

        i = 0;
        loop {
            if i >= nums.len() {
                break;
            }

            products[i] = left[i] * right[i];

            i += 1;
        }

        return products;
    }
}