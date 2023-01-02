use std::collections::HashMap;

fn main() {
    // the group of mubers to calculate the mode and median for them.
    let mut nums = vec![3, 7, 8, 4, 9, 1, 1, 1];

    // to store the mode
    let mut mode = 0;
    // sometimes there are groups of numbers that doesn't have a mode
    let mut is_there_mode: bool = false;
    // to store the median
    let median: f32;

    /* calculating the median */
    nums.sort();
    let nums_count = nums.len();

    println!("{nums:?}");

    if nums_count % 2 == 0 {
        let pre_median = nums[(nums_count / 2) - 1] as f32;
        let post_median = nums[nums_count / 2] as f32;
        median = (pre_median + post_median) / 2.0;
    } else {
        median = nums[((nums_count - 1) / 2)] as f32;
    }
}
