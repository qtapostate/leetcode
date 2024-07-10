impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if !format!("{}{}", str1, str2).as_str().eq(format!("{}{}", str2, str1).as_str()) {
            return String::from("");
        }

        let mut a = str1.len();
        let mut b = str2.len();

        loop {
            if b == 0 {
                break;
            }

            let t = b;
            b = a % b;
            a = t;
        }

        return String::from(&str1.as_str()[..a])
    }
}