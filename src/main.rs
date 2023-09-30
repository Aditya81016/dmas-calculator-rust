// importing modules
use crate::modules::methods::calculate;
use crate::modules::utils::input;

// declaring modules folder as module
mod modules;

fn main() {
    // variable to keep note on number of prompts made
    let i = 0;
    // loop through until user exits
    loop {
        // input() takes user input
        let equation = input(format!("\n{i}. Enter your problem: ").as_str(), "exit");
        // let equation = "  12 +  6 /3*   4-  2 ";

        // calculate() method returns the solution of the given equation
        let solution = calculate(equation.to_owned());

        // printing the solution
        println!("{}", solution);
    }
}
