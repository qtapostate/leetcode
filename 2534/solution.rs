impl Solution {
    pub fn time_taken(arrival: Vec<i32>, state: Vec<i32>) -> Vec<i32> {
        let mut enter_queue = std::collections::VecDeque::new();
        let mut exit_queue = std::collections::VecDeque::new();

        let mut crossed = vec![-1; arrival.len()]; // result array

        // populate the queues on each side using a VecDeque to represent the lines
        for (i, e) in arrival.iter().enumerate() {
            match state[i] {
                0 => enter_queue.push_back((i, e)),
                1 => exit_queue.push_back((i, e)),
                _ => {}
            }
        }

        // debugging
        // println!("enter queue: {:?}", enter_queue);
        // println!("exit queue: {:?}", enter_queue);

        // track the reference of the person who is entering/exiting next ("on deck")
        let mut entering: Option<(usize, &i32)> = None;
        let mut exiting: Option<(usize, &i32)> = None;

        let mut second = 0;
        let mut last_direction = -1;

        // run loop until there are no queued people, and anybody waiting has crossed
        // this accounts for holding an already-popped reference in entering/exiting respectively
        while enter_queue.len() > 0 || exit_queue.len() > 0 || entering.is_some() || exiting.is_some() {

            // identify who is next to cross on each side

            if entering.is_none() && enter_queue.len() > 0 && *(enter_queue.front().unwrap()).1 <= second {
                entering = enter_queue.pop_front();
            }

            if exiting.is_none() && exit_queue.len() > 0 && *(exit_queue.front().unwrap()).1 <= second {
                exiting = exit_queue.pop_front();
            }

            // apply rules to evaluate who is next to cross through the door

            match (entering, exiting) {
                (Some(en), Some(ex)) => { // waiting to enter: YES, waiting to exit: YES
                    match last_direction {
                        0 => { // somebody used the door in prev second to enter
                            // prefer entering
                            crossed[en.0] = second;
                            last_direction = 0;
                            entering = None;
                        },
                        1 => { // somebody used the door in prev second to exit
                            // prefer exiting
                            crossed[ex.0] = second;
                            last_direction = 1;
                            exiting = None;
                        },
                        _ => { // nobody used the door in prev secod
                            // prefer exiting
                            crossed[ex.0] = second;
                            last_direction = 1;
                            exiting = None;
                        }
                    }
                },
                (Some(en), None) => { // waiting to enter: YES, waiting to exit: NO
                    crossed[en.0] = second;
                    last_direction = 0;
                    entering = None;
                },
                (None, Some(ex)) => { // waiting to enter: NO, waiting to exit: YES
                    crossed[ex.0] = second;
                    last_direction = 1;
                    exiting = None;
                },
                (None, None) => { // waiting to enter: NO, waiting to exit: NO
                    last_direction = -1;
                }
            }

            // debugging
            // println!(
            //     "second={}, direction={}, entering={:?}, enter_queue.length={}, exiting={:?}, exit_queue.length={}",
            //     second,
            //     last_direction,
            //     entering,
            //     enter_queue.len(),
            //     exiting,
            //     exit_queue.len()
            // );

            second += 1;
        }

        return crossed;
    }
}