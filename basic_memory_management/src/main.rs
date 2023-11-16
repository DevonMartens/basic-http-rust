fn main() {
    // create an integer with the value of 2
    let a = 2;
    // then call the stack only function - passing in the value of a
    let result = stack_only(a);
    dbg!(result);
}

// stores two variables on the stack
// function out of scope so are the variables
// stack size is limited - so is the amount of memory we can use for variables - if it is too high stack overflow error 
fn stack_only(b: i32) -> i32 {
    let c = 3; // stored on stack along with b (passed in parameter)
    return b + c + stack_and_heap(); // call stack_and_heap function
}

fn stack_and_heap() -> i32 {
    let d = 5; // stored on stack
    //We create a Box<i32> called e and allocate memory for an integer with the value 7 on the heap. 
    let e = Box::new(7); // pointer to heap - stored on stack (allocated on heap)
    // Box, Arc, Rc are examples of smart pointers
    return d + *e;
}

// The dbg! macro in Rust is a debugging aid that helps you inspect and print the values of variables or expressions during program execution. 

// Checks the state of the program withoutand no need for println! 

// The dbg! macro is a shorthand for printing the value of a variable using the Debug trait.