fn main() {
    // Let - by default declares an immutable (not changeable) variable which can be shadowed.
    let x: i32 = 1111; // With type annotation/
    let x = 1111; // Type will be inferred.
                  // Adding a 'mut' keyword befor the variable declaration will allow it to be changed.
    let mut x = "I can be mutable!";
    println!("The value of x is {}.", x);

    // Const - used to declare a constant. By default, all constants are immutable.
    // You need to explicitly add a type annotation to a constant.
    // May only be set to a constant expression, not the result of a function call.
    const MY_CONSTANT: bool = true;
    println!("My constant is {}.", MY_CONSTANT);

    // Integers - Numbers without a fractional component.
    // Signed - can have a sign in front of it.
    // Unsigned - Cannot be negative only positive.

    // 8-bit
    let num1: i8 = -10; // Signed, from -128 to 127
    let num2: u8 = 10; // Unsigned, from 0 to 255
    println!("8-vit integer.\nSigned: {}\nUnsigned: {}", num1, num2);

    // 16-bit
    let num1: i16 = -30000; // Signed, from -32,768 to 32,767
    let num2: u16 = 30000; // Unsigned, from 0 to
    println!("16-bit integer.\nSigned: {}\nUnsigned: {}", num1, num2);

    // 32-bit Integer
    // By default, Rust uses this type when a number is inferred.
    let num1: i32 = -1200000; // Signed, from -2,147,483,648 to 2,145,483,647
    let num2: u32 = 2000000; // Unsigned, from 0 to 4,294,967,295
    println!("32-bit integer.\nSigned: {}\nUnsigned:{}", num1, num2);

    // 64 bit integer
    let num1: i64 = -8000000000000000; // Signed, from −(2^63) to 2^63 − 1
    let num2: u64 = 9000000000000000000; // Unsigned, from 0 to 2^64 − 1
    println!("64 bit integer.\nSigned: {}\nUnsigned: {}", num1, num2);

    // 128-bit integer
    let num1: i128 = -170141183460469231731687303715884105728; // Signed, from -(2^127) to 2^127 -1
    let num2: u128 = 340282366920938463463374607431768211455; // Unsigned from 0 to 2^128 -1
    println!("128 bit integer.\nSigned: {}\nUnsigned: {}", num1, num2);

    // Array - A collection of multiple values.
    let array = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    println!("{}", array[0]) // Printing first element of array. Arrays are zero indexed.
}
