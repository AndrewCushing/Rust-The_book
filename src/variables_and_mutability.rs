fn mutable() {
    let x = 5; // Variables are immutable by default

    // We cannot now do x = 6 as the compiler will throw an error

    let mut y = 4; // This is how to make a mutable variable

    y = 9; // Now this is perfectly valid

    const Z: u8 = 8; // Constants are declared with the const keyword, can never be mutable, must have their type annotated and must be given a value immediately.

    // Shadowing

    let a = 7;

    let a = 9; // We say this variable 'shadows' the original one. Whenever we access a from now on, we'll get this value

    let a = "blah"; // We can even use shadowing to make a have a different type, just to make things more confusing
    // Don't forget a is still immutable so we might well be relying on it not changing
}