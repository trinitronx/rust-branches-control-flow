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
}