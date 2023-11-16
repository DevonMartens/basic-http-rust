use std::io; // io module from the standard library, which is used to perform input and output operations.

fn main() {
    println!("Enter your weight (kg): ");

    let mut input = String::new(); // create a mutable variable called input and bind it to a new, empty instance of a String

    io::stdin().read_line(&mut input).unwrap(); // This line prints a message to the console, prompting the user to enter their weight in kilograms.

    // This line takes the user's input (which is a string), trims any leading or trailing whitespace, 
    // parses it as a floating-point number (f32), and stores the result in the weight variable.
    let weight: f32 = input.trim().parse().unwrap(); // This line reads the user’s input from the console and stores it in the input variable.

    // The program calls the calculate_weight_on_uranus function, passing the Earth weight as an argument, and stores the result in the uranus_weight variable.
    let uranus_weight = calculate_weight_on_uranus(weight);
    //prints weight
    println!("Weight on uranus: {}kg", uranus_weight);
}

// Takes a person's weight on Earth as an argument (weight). 
// Calculates the weight on uranus using the formula (weight / 9.81) * 3.711 and returns the result as an f32 floating-point number.
fn calculate_weight_on_uranus(weight: f32) -> f32 {
    (weight / 9.81) * 8.69
}