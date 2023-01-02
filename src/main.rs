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

    // calculating the mode
    let mut nums_repeating_count: HashMap<i32, i32> = HashMap::new();

    // using a hashmap to record
    // how many each item in nums is repeated
    for num in &nums {
        // if the num doesn't exist in the map we insert 0
        // if it exists we return a mut reference to its value
        // then increase the value by one
        let count = nums_repeating_count.entry(*num).or_insert(0);
        *count += 1;
    }

    // fetching the most repeated item (mode)
    for (key, value) in &nums_repeating_count {
        // if the value is bigger than the value of the current mode
        // the mode is assigned the current key
        // using unwrap_or(1) is to prevent using a num that 's only repeated once
        if *value > nums_repeating_count.get(&mode).copied().unwrap_or(1) {
            is_there_mode = true;
            mode = *key;
        }
    }

    println!("the median is: {median}");
    if is_there_mode {
        println!("the mode is: {mode}");
    } else {
        println!("there is no mode")
    }
}
