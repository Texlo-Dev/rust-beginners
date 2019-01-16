fn main() {
    // If statements - Most basic conditional. Does something if something is true.
    let x = 2;
    if x > 2 {
        // No () around the statement required.
        println!("The number condition is satisfied!");
    }
    // Else if - Allows you to combine if and else to have multiple conditions.
    else if x == 2 {
        println!("The number condition was exactly met!");
    }
    // Else - Will always run if the above condition was not satisfied.
    else {
        println!("The condition was not satisfied!");
    }

    // let ... if condition allows us to easily bind the result of a condition to a variable.
    let result = if x == 2 {
        "Result variable is now this!"
    } else {
        "Result variable could also be this!"
    };
    println!("{}", result);
}
