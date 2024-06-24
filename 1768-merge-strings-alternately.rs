impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut i = 0;
        let mut c = 0;
        let mut final_string: String = String::with_capacity(word1.len() + word2.len());

        loop {
            if i == word1.len() + word2.len() {
                break;
            }

            match i % 2 {
                0 => {
                    if c < word1.len() {
                        final_string.push_str(&word1[c..c+1]);
                    } else {
                        final_string.push_str(&word2[c..]);
                        break;
                    }
                },
                1 => {
                    if c < word2.len() {
                        final_string.push_str(&word2[c..c+1]);
                    } else {
                        final_string.push_str(&word1[c+1..]);
                        break;
                    }

                    c += 1;
                },
                _ => {
                    break;
                }
            }

            i += 1;
        }

        return final_string;
    }
}