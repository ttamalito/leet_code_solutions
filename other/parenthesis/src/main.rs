fn main() {
    println!("Hello, world!");
    let input = String::from("()[]{}");
    println!("{}",is_valid(input));
}

pub fn is_valid(s: String) -> bool {
        let one = '(';
        let two = ')';
        let three = '[';
        let four = ']';
        let five = '{';
        let six = '}';

        let mut iterator = s.as_str().chars();
        let mut previous: char = iterator.next().unwrap();
        // iterate through every single character
        for i in iterator {
            if i == two && previous == one {
                previous = i;
                println!("We found ()");
                continue;
            } else if i == two && previous != one {
                println!("False 1");
                return false;
            }
            if i == four && previous == three {
                previous = four;
                println!("We found []");
                continue;
            } else if i == four && previous != three {
                println!("False 2");
                return false;
            }

            if i == six && previous == five {
                previous = six;
                println!("We found The third type");
                continue;
            } else if i == six && previous != five {
                println!("False 3");
                return false;
            }

            if i == one && (previous == three || previous == five) {
                // it is false
                return false;
            }
            
            if i == three && (previous == one || previous == five) {
                return false;
            }

            if i == five && (previous == one || previous == three) {
                return false;
            }
            previous = i;
        } // end of for loop
        true
}