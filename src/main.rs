fn main() {
    // let number = 3; // condition was true
    let number = 7; // condition was false

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Does NOT compile!
    // expected `bool`, found integer
    // if number {
    //     println!("number was three");
    // }

    // Explicit boolean expressions must be used
    if number != 0 {
        println!("number was something other than zero");
    }

    // Handling Multiple Conditions with else if
    multiple_conditions();

    // Using if in a let Statement
    if_in_let_statement();
}

// Example of multiple if .. else if .. else conditions
fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Example of Using if in a let Statement
fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // Does not compile - `if` and `else` have incompatible types
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
