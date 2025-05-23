// This section demonstrates how Rust handles integer operations.
fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1f64 + 2.55); // The `+` operator is used for addition.
    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2); // The `-` operator is used for subtraction.
    // When we change the type of one of the operands to `1u32`, Rust will perform unsigned integer subtraction, which can result in a negative number. However, this is not what we want here. We want signed integer subtraction.
    // Scientific notation
    println!("1e4 is {}", 1e4); // Exponentiation is denoted by the letter 'e' followed by the exponent.
    println!("-2.5e-3 is {}", -2.5e-3); // A negative sign can be placed before or after the number, but it's more conventional to place it before.
    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false); // The `&&` operator represents a logical 'and'.
    println!("true OR false is {}", true || false); // The `||` operator represents a logical 'or'.
    println!("NOT true is {}", !true); // The `!` symbol represents a logical NOT operation.
    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // In binary, the '&' operator performs bitwise 'and'.
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // The '|' symbol represents a bitwise 'or' operation.
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // The '^' operator performs an exclusive-or (XOR) operation on the bits of two numbers.
    println!("1 << 5 is {}", 1u32 << 5); // This expression shifts the binary representation of `1` to the left by `5` places, equivalent to multiplying it by `2^5`.
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // The '>>' operator shifts the bits of a number to the right. We can use this with hexadecimal literals, but note that we are using `u32` instead of just `0x80`.
    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32); // By placing an underscore between each group of three digits, the number becomes easier to read.
}