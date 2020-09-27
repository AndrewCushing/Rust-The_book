pub fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1u64
    } else {
        fibonacci(n - 1) + fibonacci( n - 2)
    }
}