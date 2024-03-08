fn main() {
    println!("Hello, world!");
    let res = max_frequency_elements(vec![1,2,3,4,5]);
    println!("{}", res);
}


pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    // get the maximum element of nums
    let mut max_num = -1;
    for i in &nums {
        if *i > max_num {
            max_num = *i;
        }
    }
    // create a new array
    let mut frequencies: Vec<i32> = vec![0; (max_num + 1).try_into().unwrap()]; // initialize an array so store all the frequencies
    let mut maximum: Vec<(usize, i32)> = vec![(0, -1)]; // (element, count)

    // iterate through the array
    for i in nums {
        let converted: usize = (i).try_into().unwrap();
        // update the counter
        frequencies[converted] += 1;

    } // end of for loop

    for (element, count) in frequencies.iter().enumerate() {
        if *count == 0 {
            continue; // go to the next iteration
        }

        if *count > maximum[maximum.len() -1].1 {
            // we have a bigger count, so clean the array
            maximum.clear();
            maximum.push((element, *count));
        } else if *count == maximum[maximum.len() - 1].1 {
            // they both have the same count
            // so add the element to the array
            maximum.push((element, *count));
        }
    }

    // now iterate through maximum to find the maximum element
    let mut result = 0;
    for (_element, count) in maximum {
        result += count;
    }
    result
}
