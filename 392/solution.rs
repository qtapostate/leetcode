// beats 100% time, 91.30% space
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut i = 0;
        let mut j = 0;

        return loop {
            if j == s.len() {
                break true;
            } else if i == t.len() {
                break false;
            }

            if s[j..j+1] == t[i..i+1] {
                j += 1;
            }
            i += 1;
        }
    }
}