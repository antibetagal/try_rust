// Comments are used to explain parts of the code. This line explains that tuples can be used as function arguments and return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // The `let` keyword is used to bind the members of a tuple to variables. In this case, we're binding the first element of the tuple to `int_param` and the second element to `bool_param`.
    let (int_param, bool_param) = pair;

    // We're returning a new tuple with the boolean parameter as the first element and the integer parameter as the second element.
    (bool_param, int_param)
}

// The `#[derive(Debug)]` attribute is used to derive the `Debug` trait for the struct. This allows us to use the `{?}` format specifier to print the contents of the struct.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // We're creating a tuple with a bunch of different types. The first three elements are unsigned integers (u8, u16, and u32), followed by signed integers (-1i8, -2i16, and -3i32). Then we have floating-point numbers (0.1f32 and 0.2f64) and characters ('a' and true).
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // We're using the `.` operator to access the first element of the tuple.
    println!("Long tuple first value: {}", long_tuple.0);

    // We're using the `.` operator again to access the second element of the tuple.
    println!("Long tuple second value: {}", long_tuple.1);

    // We're creating a new tuple where each element is itself another tuple. This is an example of nesting tuples.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // We're using the `{?}` format specifier to print the contents of the `tuple_of_tuples` variable. This is only possible because we derived the `Debug` trait for tuples.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // We're creating a new tuple with more than 12 elements, which means it's considered "too long" and can't be printed using the `{?}` format specifier.
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, /*9,*/ 10, 11, 12, 13);

    // We're trying to print the contents of `too_long_tuple` using the `{?}` format specifier. However, this will result in a compiler error because tuples with more than 12 elements can't be printed this way.
    println!("Too long tuple: {:?}", too_long_tuple);

    // We're creating a new pair with an integer and a boolean value.
    let pair = (1, true);

    // We're printing the contents of `pair` using the `{?}` format specifier.
    println!("Pair is {:?}", pair);

    // We're calling the `reverse` function with `pair` as an argument and storing the result in the `reversed_pair` variable.
    let reversed_pair = reverse(pair);

    // We're printing the contents of `reversed_pair` using the `{?}` format specifier.
    println!("The reversed pair is {:?}", reversed_pair);

    // We're creating a new one-element tuple with an integer value. Note that we need to include a trailing comma after the value to tell Rust it's a tuple and not just a literal surrounded by parentheses.
    let one_element_tuple = (5u32,);

    // We're printing the contents of `one_element_tuple` using the `{?}` format specifier.
    println!("One element tuple: {:?}", one_element_tuple);

    // We're creating another new one-element tuple with an integer value. This time we're not including a trailing comma after the value, so Rust will just treat it as a literal surrounded by parentheses.
    let just_an_integer = 5u32;

    // We're printing the contents of `just_an_integer` using the `{?}` format specifier. Note that this will print an integer, but the tuple itself won't be printed.
    println!("Just an integer: {:?}", just_an_integer);

    // We're creating a new tuple with four elements: an integer, a string, a floating-point number, and a boolean value.
    let tuple = (1, "hello", 4.5, true);

    // We're using the `let` keyword to bind each element of the tuple to its own variable.
    let (a, b, c, d) = tuple;

    // We're printing the contents of each variable using the `{}` format specifier.
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // We're creating a new `Matrix` struct with four floating-point numbers as its fields.
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    // We're printing the contents of `matrix` using the `{?}` format specifier. Note that this will print a string representation of the struct, which includes all four fields.
    println!("{:?}", matrix);
}