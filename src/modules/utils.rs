// import io for taking input
use std::io;

// this method takes a msg and err to prompt user or a input and print error message if any error
// occured
pub fn input(msg: &str, err: &str) -> String {
    // declaring variable value of type String to store user input
    let mut value = String::new();

    // prints the msg provided to prompt user
    println!("{msg}");
    // stores the user input in value else if any error occured then prints the error message
    io::stdin().read_line(&mut value).expect(&err);

    // returns value which stores the user input
    value
}

// this method returns the index of given value in given vector
pub fn index_of(vector: Vec<String>, value: String) -> i32 {
    // turns vector into a interator and compares each item in the vector to the given value if and
    // of the item matched then the index of that item is stored in index and then the index is
    // returned else it comes out of if statement and returns -1 which indicates the value was not
    // found in vector
    if let Some(index) = vector.iter().position(|item| item == &value) {
        return index as i32;
    }
    -1
}

// converts a vector equation into a statement
pub fn get_statement(equation: &Vec<String>) -> String {
    // creates a new String variable statement
    let mut statement = String::new();
    // converts equation into an iterator and goes through each element in the equation and
    for element in equation.iter() {
        // adds the element into statement followed by a space
        statement = statement + element + " ";
    }

    // returns the generated statement
    statement
}

// returns true if given string is numeric else false
pub fn is_numeric(string: &str) -> bool {
    // declare a boolean mutable variable valid
    let mut valid = false;

    // try to parse string into i32 (32-bit integer type) and if it is parsed successfully then
    if let Ok(_) = string.parse::<f32>() {
        // update valid to be true
        valid = true;
    }
    // return valid
    valid
}
