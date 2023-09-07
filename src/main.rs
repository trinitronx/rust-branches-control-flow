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

    // Repetition with Loops
    // repetition_with_loops(); // Uncomment for infinite loop demo

    // Returning Values from Loops
    returning_values_from_loops();

    // Loop Labels to Disambiguate Between Multiple Loops
    break_outer_from_nested_loop();

    // Conditional Loops with while
    while_loop();

    // Looping Through a Collection with for
    looping_through_collections();
    looping_through_collections_for_in_loop();
    for_number_in_range_loop();
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

// Example of Repetition with Loops
fn repetition_with_loops() {
    // infinite loop
    loop {
        println!("again!");
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

// Example of Returning Values from Loops
fn returning_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}"); // The result is 20
}

// Example: Loop Labels to Disambiguate Between Multiple Loops
fn break_outer_from_nested_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// While loop example
fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// Example of Looping Through a Collection with for
// static condition check
fn looping_through_collections() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Could also use: while index < a.len()
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// Example of Looping Through a Collection with for
// for .. in ..
fn looping_through_collections_for_in_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// Example of Looping Through a Collection with for number in range
// for .. in (N..M)
fn for_number_in_range_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}