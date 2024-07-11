impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words: Vec<_> = s.split(" ").filter(|&v| v.len() > 0).collect();
        let mut reversed = Vec::<&str>::new();

        let mut i = 0usize;

        if words.len() == 0 {
            return "".to_string();
        }

        i = words.len() - 1;
        
        loop {
            reversed.push(words[i]);

            if i == 0 {
                break;
            }

            i -= 1usize;
        }

        return reversed.join(" ");
    }
}