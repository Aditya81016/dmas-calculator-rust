// import methods from utils and std
use super::utils::{get_statement, index_of, is_numeric};

// formats the equation into desired format
pub fn format(equation: &str) -> String {
    // create variable for formated equation
    let mut formated_equation: String = String::from("");

    // concise the equation and store it in concised_equation variable
    let concised_equation = equation.trim().replace(" ", "");

    // convert concised_equation into a character iterator and go through each character and...
    concised_equation.chars().for_each(|char| {
        // check if character is numeric then...
        if char.is_numeric() || char == '.' {
            // push the character into formated_equation
            formated_equation.push(char);
        } else {
            // else push char with 1-1 trailing spaces on both side using format method
            formated_equation.push_str(format!(" {char} ").as_str());
        }
    });

    // return the formated_equation
    formated_equation
}

// splits the equation into a vector
pub fn split(equation: &str) -> Vec<String> {
    // create a new variable with a space at the end of equation
    let equation_with_space = equation.to_string() + " ";

    // create a vector to store vectorized equation
    let mut vector = Vec::new();
    // create a string variable set to store element for the vector
    let mut set = String::new();

    // go through each character of equation with a space and
    equation_with_space.chars().for_each(|char| {
        // check if a character is a whitespace then
        if char.is_whitespace() {
            // push the set inside of vector
            vector.push(set.clone());
            // and clear the set to store new element
            set.clear();
        } else {
            // elese push the char into set to build the element
            set.push(char);
        }
    });

    // return the vector
    vector
}

// returns true if the vector equation is valid or not
pub fn verify(equation: &Vec<String>) -> bool {
    // declare a boolean variable valid
    let mut valid = true;

    // create a vector containing the operator
    let operators = vec!["/", "*", "+", "-"];

    // to keep track of sequence of number and operator in the equation
    let mut count = 0;

    // go through each element in the equation
    for element in equation {
        // if the element is numeric and count is 0 (which means either it is start of equation or
        // an operator of before this) then...
        if is_numeric(&element.as_str()) && count == 0 {
            // add 1 to count
            count = count + 1;
        }
        // else if the element is an operator and count is 1 (which means a number was before this)
        // then...
        else if operators.contains(&element.as_str()) && count == 1 {
            // subract 1 from count
            count = count - 1;
        }
        // else if it was neither an operator nor a number then equation had an invalid character
        // then...
        else {
            // set valid to false
            valid = false;
        }
    }

    // if equation have valid character but the count is not 1 (which means sequence of number and
    // operator wasn't right) then...
    if valid && count != 1 {
        // set valid to false
        valid = false;
    }

    // return valid
    valid
}

// calculates a vector equation using recursion
pub fn calculate_vec(mut equation: Vec<String>, mut solution: String) -> String {
    // vector containing valid operators
    let operators = vec!["/", "*", "+", "-"];

    // variables to store the result of operation and the generated statement for this step
    let result;
    let statement;

    // go through operators in operators variable in sequence which is DMAS
    for operator in operators {
        // check if equation contains the operator
        if equation.contains(&operator.to_owned()) {
            // get the index of the operator
            let index_of_operator = index_of(equation.clone(), operator.to_owned());

            // get the number before the operator
            let no_before_operator = match equation.get((index_of_operator - 1) as usize) {
                Some(value) => value.parse::<f32>().unwrap_or(-1.0),
                None => -1.0,
            };
            // get the no after operator
            let no_after_operator = match equation.get((index_of_operator + 1) as usize) {
                Some(value) => value.parse::<f32>().unwrap_or(-1.0),
                None => -1.0,
            };

            // perform respective operation on number before operator by number after operator based on what the operator was and store the result in
            // the result variable
            if operator == "/" {
                result = (no_before_operator / no_after_operator).to_string();
            } else if operator == "*" {
                result = (no_before_operator * no_after_operator).to_string();
            } else if operator == "+" {
                result = (no_before_operator + no_after_operator).to_string();
            } else {
                result = (no_before_operator - no_after_operator).to_string();
            }

            // remove the operator and number after it
            equation.remove(index_of_operator as usize);
            equation.remove(index_of_operator as usize);
            // replace the number before operator by result
            equation[(index_of_operator - 1) as usize] = result;

            // generate a statement and store it in statement variable
            statement = get_statement(&equation);
            solution = solution + statement.as_str() + "\n";

            // trigger recursion
            return calculate_vec(equation, solution);
        }
    }

    // return the final solution when no more operator was left in equation
    solution
}

// this function calls every other function here to calculate a given string equation
pub fn calculate(equation: String) -> String {
    // creates a variable to store solution
    let mut solution = String::from("Solution \n");

    // format() function formats equation in a desired format
    let formated_equation = format(&equation[..]);

    // add the formated equation in the solution
    solution = solution + formated_equation.as_str() + "\n";

    // convert the string equation into a vector
    let equation_vector = split(formated_equation.as_str());

    // if the equation is valid then
    if verify(&equation_vector) {
        // return the solution else
        solution + calculate_vec(equation_vector, "".to_owned()).as_str()
    } else {
        // return a message to notify user the given equation was invalid
        String::from("Invalid Equation")
    }
}
