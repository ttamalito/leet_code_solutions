fn main() {
    println!("Hello, world!");
    let result = two_sum(vec![3,2,4], 6);
    for i in result {
        println!("{}", i);
    }
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    println!("We are running two_sums");
    let mut index_one: Box<i32>=  Box::new(0);
    let mut index_two: Box<i32> = Box::new(0);
    println!("nums length: {}", nums.len());
 // create a for loop
 for i in 0..nums.len() {
    println!("We are running the first loop");
    // do a second iteration
    for j in 0..nums.len() {
        // check if they are the same index
        if j == i {
            continue;
        }
        let sum: i32 = nums[i] + nums[j];
        if sum == target {
            println!("We have a match");
            // we have the two indices
            *index_one = (i).try_into().unwrap();
            *index_two = (j).try_into().unwrap();
            return vec![*index_one, *index_two];
        }
    } // end of inner loop
 }       // end of outer loop

 vec![0; 2]
}