fn main() {
    println!("{}", generate_fibonacci_number(45));
}

fn generate_fibonacci_number(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }
    generate_fibonacci_number(n - 1) + generate_fibonacci_number(n - 2)
}
