use crate::challenges::chapter_8::mean_median_mode::get_stats;

pub fn run(){
    test1();
    test2();
    test3();
}

fn test1(){
    let mut nums: Vec<i32> = vec![1, 2, 3];
    let (mean, median, mode) = get_stats(&mut nums);
    println!("For the nums: {:?}", nums);
    println!("Mean: {}, median: {}, mode: {}", mean, median, mode);
}

fn test2(){
    let mut nums: Vec<i32> = vec![8735, 2354234, 3452354, 3425324, 53245, 32454, 2354, -873487, 0, 0];
    let (mean, median, mode) = get_stats(&mut nums);
    println!("For the nums: {:?}", nums);
    println!("Mean: {}, median: {}, mode: {}", mean, median, mode);
}

fn test3(){
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let (mean, median, mode) = get_stats(&mut nums);
    println!("For the nums: {:?}", nums);
    println!("Mean: {}, median: {}, mode: {}", mean, median, mode);
}