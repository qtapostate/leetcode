impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result: Vec<bool> = vec![false; candies.len()];
        let mut greatest = 0;

        let mut i = 0;

        // find greatest value by looping through n elements and finding greatest value
        // improvement on previous O(nlogn) worst-case
        loop {
            if i >= candies.len() {
                break;
            }

            if candies[i] > greatest {
                greatest = candies[i];
            }

            i += 1;
        }

        i = 0; // reuse and reset counter for next loop

        println!("greatest: {}", greatest);

        // mark those who have the greatest
        loop {
            if i >= candies.len() {
                break;
            }

            if candies[i] + extra_candies >= greatest {
                result[i] = true;
            }

            i += 1;
        }

        return result;
    }
}
