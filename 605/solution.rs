impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut plantable_count = 0;
        let mut plantable = false;

        let mut i = 0;

        let mut window = (None, None, None);

        loop {
            window = (flowerbed.get(i-1), flowerbed.get(i), flowerbed.get(i+1));

            plantable = match window {
                (Some(prev), Some(curr), Some(next)) => *prev == 0 && *curr == 0 && *next == 0,
                (Some(prev), Some(curr), None) =>  *prev == 0 && *curr == 0,
                (None, Some(curr), Some(next)) => *curr == 0 && *next == 0,
                (None, Some(curr), None) => *curr == 0,
                _ => break,
            };

            if plantable {
                i += 1;
                plantable_count += 1;
                plantable = false;
            }

            i += 1;
        }

        return plantable_count >= n;
    }
}