impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 1 {
            return 1i32;
        }

        let len = chars.len(); // original length of array
        let mut i = 1;
        let mut group = 1;
        let mut c = chars[0];

        loop {
            if i >= len {
                chars.append(&mut vec![c]);
                
                if group > 1 {
                    chars.append(&mut group.to_string().as_str().chars().collect::<Vec<_>>());
                }

                break;
            }

            if c == chars[i] {
                group += 1
            } else {
                chars.append(&mut vec![c]);

                if group > 1 {
                    chars.append(&mut group.to_string().as_str().chars().collect::<Vec<_>>());
                }

                c = chars[i];
                group = 1;
            }

            i += 1;
        }
        
        chars.drain(..len);

        return chars.len() as i32;
    }
}