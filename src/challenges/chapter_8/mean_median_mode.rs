//Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode (the value that occurs
// most often; a hash map will be helpful here) of the list.
use std::collections::HashMap;
use std::cmp::max;
use std::borrow::Borrow;

pub fn get_stats(nums: &mut Vec<i32>) -> (f32, f32, i32) {
    nums.sort();

    let mut mode_collection: HashMap<i32, u32> = HashMap::new();
    let mut total = 0;
    let length = Vec::len(&nums);
    let mut median: f32 = 0.0;

    for &num in &nums[..] {
        let count = mode_collection.entry(num).or_insert(0);
        *count += 1;
        total += num;
    };
    if length % 2 == 0 {
        median = (nums[length/2] + nums[length/2 - 1]) as f32 / 2 as f32;
    } else {
        median = nums[length/2] as f32;
    };
    let mut mode = 0;
    let mut max_freq = 0;

    for (num, freq) in mode_collection {
        if freq > max_freq {
            mode = num;
            max_freq = freq;
        }
    };

    let mean: f32 = total as f32 / length as f32;

    (mean, median, mode)
}