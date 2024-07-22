impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut start: (usize, i32) = (0, height[0]);
        let mut end: (usize, i32) = (height.len() - 1, height[height.len() - 1]);
        let mut a: i32 = 0;
        let mut t = 0;

        while end.0 - start.0 != 0 {
            t = match start.1 < end.1 {
                true => start.1 * ((end.0 - start.0) as i32),
                false => end.1 * ((end.0 - start.0) as i32)
            };

            if t > a {
                a = t;
            }

            if start.1 < end.1 {
                start = (start.0 + 1, height[start.0 + 1]); // move start to the right
            } else {
                end = (end.0 - 1, height[end.0 - 1]); // move end to the left
            }
        }

        return a;
    }
}