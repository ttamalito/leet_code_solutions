fn main() {
    println!("Hello, world!");
    roman_to_int(String::from("III"));
}


    pub fn roman_to_int(s: String) -> i32 {
        let mut value = 0;
        let mut substract = 0;
        // iterate through all the chars
        let slice = s.as_str();
        let what: Vec<&str> = slice.split("").collect();
        // iterate through all the chars
        for (index, c) in what.iter().enumerate() {
            match **c {
                "M" => {
                    // check the index
                    if index == 1 {
                        // just add M to the value
                        value += 1000;
                    } else {
                        // check if there is something to substract
                        if substract == 0 {
                            // nothing to substract, i.e. add 1000
                            value += 1000;
                        } else {
                            // there is something to substract, 
                            let to_add = 1000 - substract;
                            value += to_add;
                            substract = 0;
                        }
                    }
                }; // end of M
                "D" => {
                    // really similar to M
                    // check the index
                    if index == 1 {
                        // just add M to the value
                        value += 500;
                    } else {
                        // check if there is something to substract
                        if substract == 0 {
                            // nothing to substract, i.e. add 500
                            value += 500;
                        } else {
                            // there is something to substract, 
                            let to_add = 500 - substract;
                            value += to_add;
                            substract = 0;
                        }
                    }
                };
                "C" => {
                    // check the index
                    if index == 1 {
                        // just add M to the value
                        value += 500;
                    } else {
                        // check if there is something to substract
                        if substract == 0 {
                            // nothing to substract, i.e. add 500
                            value += 500;
                        } else {
                            // there is something to substract, 
                            let to_add = 500 - substract;
                            value += to_add;
                            substract = 0;
                        }
                    }
                };
                
            } // end of match
        }
        9
    }
