impl Solution {
    pub fn is_slice_vowel(slice: &str) -> bool {
        return match slice {
            "a" | "e" | "i" | "o" | "u" => true,
            _ => false
        };
    }
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let n = k as usize;
        let mut s = s.as_str();
        let mut count = 0;

        for i in 0..=n-1 {
            if Solution::is_slice_vowel(&s[i..=i]) {
                count += 1;
            }
        }

        let mut max = count;

        for j in n..=s.len()-1 {
            match (Solution::is_slice_vowel(&s[(j-n)..=(j-n)]), Solution::is_slice_vowel(&s[j..=j])) {
                (true, false) => count -= 1,
                (false, true) => count += 1,
                _ => {}
            }

            if count > max {
                max = count;
            }
        }
        return max;
    }
}