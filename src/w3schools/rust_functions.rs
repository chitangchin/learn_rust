pub fn print_rust_function() {
    println!("\nThis is the rust_function.rs file****");
    say_hello();
    let sum: i32 = add(3,4);
    println!("{}", sum);
    let diff: i32 = sub(10, 7);
    println!("{}", diff);
}

fn say_hello() {
    println!("Hello from function!");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}