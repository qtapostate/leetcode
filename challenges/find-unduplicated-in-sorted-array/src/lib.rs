pub fn find_unique_value(input: &[i64]) -> i64 {
    let bisected = split(input);

    println!("{:?}", bisected);

    let left_len = bisected[0].len();
    let right_len = bisected[1].len();

    // check the base cases

    if left_len == 1 { // base case: return element on left
        return bisected[0][0];
    }

    if right_len == 1 { // base case: return element on right
        return bisected[1][0];
    }

    // choose which direction to continue with
    // under ordinary conditions, we will use the direction with an odd number of elements for our next recursive iteration

    return match (left_len % 2 != 0, right_len % 2 != 0) {
        (true, false) => find_unique_value(bisected[0]),
        (false, true) => find_unique_value(bisected[1]),
        _ => 0,
    };
}

fn split(input: &[i64]) -> [&[i64]; 2] {
    let mut midpoint_index = input.len() / 2;

    if input[midpoint_index - 1] == input[midpoint_index] {
        midpoint_index -= 1;
    }

    let left = &input[0..midpoint_index];
    let right = &input[midpoint_index..];

    return [left, right];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_unique_1() {
        let input = [-1, -1, 3, 3, 4, 5, 5, 9, 9];
        assert_eq!(find_unique_value(&input), 4);
    }

    #[test]
    fn test_find_unique_2() {
        let input = [-6, -5, -5, -2, -2, 14, 14, 20, 20, 21, 21, 82, 82, 1000, 1000, 1450, 1450];
        assert_eq!(find_unique_value(&input), -6);
    }
}