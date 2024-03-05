fn main() {
    let result = is_palindrome(101);
    println!("{}", result);
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x == 0{
        return true;
    }
    let mut input = x;
    let mut digits = vec![];
    loop {
        let digit = input % 10; // final digit
        // substract that quantity
        input = if digit == 0 {
            input / 10
        } else {
            input / 10
        };
        // add the digit to the vector
        digits.push(digit);
        // check if input == 0
        if input == 0 {
            break;
        }

    } // end of the loop
    // check if the numbers in the vector are the same
    let reverse: &Vec<i32> = &digits;
    let mut index_reverse = reverse.len() - 1;
    println!("{}",index_reverse);
    let mut index = 0;
    let result = true;
    loop {
        if digits[index] != digits[index_reverse] {
            return false;
        }
        // increase index by one
        index += 1;
        // decrease index_reverse by one
        if index_reverse == 0 {
            break;
        }
        
        index_reverse -= 1;
    }

    return result;

} // end of palindrome