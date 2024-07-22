impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut minute = 0;
        let mut y = 0;
        let mut x = 0;
        let mut fresh = 0; // fresh detected in current minute m
        let mut rotted = 0; // oranges rotted in current minute m (not total count of rotten)
        let mut next_grid = grid.clone();

        loop {
            next_grid[y][x] = if grid[y][x] == 1 {
                if y > 0 && grid[y-1][x] == 2 {
                    rotted += 1;
                    2
                } else if y < grid.len() - 1 && grid[y+1][x] == 2 {
                    rotted += 1;
                    2
                } else if x > 0 && grid[y][x-1] == 2 {
                    rotted += 1;
                    2
                } else if x < grid[y].len() - 1 && grid[y][x+1] == 2 {
                    rotted += 1;
                    2
                } else {
                    fresh += 1;
                    1
                }
            } else {
                grid[y][x]
            };

            // if we are at the last row and last column
            if y + 1 == grid.len() && x + 1 == grid[y].len() { // iterated through all cells of current grid
                // check ending conditions:
                // if we have some fresh remaining and none rotted this minute: the spread has stopped, it is impossible
                // if we have no fresh and none rotted: the spread has completed, return the current minute.
                if fresh > 0 && rotted == 0 {
                    return -1;
                } else if fresh == 0 && rotted == 0 {
                    return minute;
                }

                // advance to next minute and reset
                minute += 1;
 
                y = 0;
                x = 0;
                rotted = 0;
                fresh = 0;

                grid = next_grid.clone(); // TODO: can probably be done in O(1) if we only look forward

                continue;
            }


            // advance to next column
            x = if x + 1 == grid[y].len() {
                y += 1;
                0
            } else {
                x + 1
            }
        }

        return -1;
    }
}