impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let h: i32 = (n as f32 / 2f32).trunc() as i32;
        let m: i32 = n % 2;

        let mut factors = Vec::new();

        // fill in obvious factors to reduce iterations
        match m {
            0 => factors.append(&mut vec![1, 2, h, n]),
            _ => factors.append(&mut vec![1, n])
        }

        let mut i = (2 - m) as usize;
        let mut a = 3;

        loop {
            if a > h {
                break;
            }

            if n % a == 0 {
                // if i == k as usize {
                //     return a;
                // }
                i += 1;
                factors.insert(i-1, a);
            }

            a += 1;
        }

        factors.dedup(); // lazy solution

        println!("factors: {:?}", factors);

        if k as usize > factors.len() {
            return -1;
        }

        return factors[k as usize - 1usize];
    }
}