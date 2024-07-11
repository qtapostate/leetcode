impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let chars = s.as_bytes();
        let mut pos = Vec::<Option<usize>>::with_capacity(s.len());
        let mut reversed: Vec<u8> = Vec::with_capacity(s.len());

        let mut i = 0usize;

        for ch in chars {
            match (*ch as char).to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    pos.push(Some(i)); // track index of vowel
                },
                _ => {
                    pos.push(None);
                }
            }

            i += 1usize;
        }

        let vowels: Vec<usize> = pos.iter().filter(|&ch| ch.is_some()).map(|&ch| ch.unwrap()).collect();

        i = 0usize;
        let mut v = vowels.len() - 1;

        loop {
            if i >= s.len() || v < 0 {
                break;
            }

            if let Some(vowel) = pos[i] {
                reversed.push(chars[vowels[v]].clone());
                v -= 1;
            } else {
                reversed.push(chars[i].clone());
            }

            i += 1;
        }

        return String::from_utf8(reversed).unwrap();
    }
}