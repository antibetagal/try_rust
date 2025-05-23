// This function will contain all of our program's logic.
fn main() {
    // We're demonstrating integer addition by adding 1 and 2 together.
    println!("1 + 2 = {}", 1u32 + 2);

    // Now we'll demonstrate integer subtraction, but we're using a signed type (i32) to show how it handles negative numbers.
    println!("1 - 2 = {}", 1i32 - 2);
    // If we change `1i32` to `1u32`, the program will panic because we can't subtract a larger number from a smaller one using unsigned integers.
    // This is why it's so important to use signed types when you need to handle negative numbers.

    // We're demonstrating scientific notation by printing out two examples.
    println!("1e4 is {}", 1e4);
    // Scientific notation uses 'e' or 'E' followed by a number that represents the exponent of the base number.
    println!("-2.5e-3 is {}", -2.5e-3);

    // Boolean operations are important in programming, so let's demonstrate them.
    // The `&&` operator returns true if both operands are true.
    println!("true AND false is {}", true && false);
    // The `||` operator returns true if either operand is true.
    println!("true OR false is {}", true || false);
    // The `!` operator inverts the boolean value of its operand.
    println!("NOT true is {}", !true);

    // Bitwise operations are used to manipulate individual bits within a number.
    // We're using the `&` operator to perform an AND operation on two numbers.
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // The `|` operator performs an OR operation on two numbers.
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // The `^` operator performs an XOR (exclusive or) operation on two numbers.
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // The `<<` operator shifts the bits of its left operand to the left by the number of places specified in its right operand.
    println!("1 << 5 is {}", 1u32 << 5);
    // The `>>` operator shifts the bits of its left operand to the right by the number of places specified in its right operand.
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // To make numbers like "one million" easier to read, we can use underscores to separate groups of digits.
    println!("One million is written as {}", 1_000_000u32);
}