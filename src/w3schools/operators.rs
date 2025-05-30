pub fn print_operators() {
    println!("\nThis is the operators.rs file****");
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div: f64 = 10.0 / 3.0;
    let rem: i8 = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    let mut x: i8 = 10;
    x += 117;
    println!("X after calculation += : {}", x);

    x-= 30;
    println!("X after calculation -=: {}", x);

    x /= 3;
    println!("X after calculation /=: {}", x);

    x *= 2;
    println!("X after calculation *=: {}", x);

    x %= 2;
    println!("X after calculation %=: {}", x);

    println!("\nConditionals");

    let a: u8 = 5;
    let b: u8 = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 > 10: {}", a > b);

    let logged_in: bool = true;
    let is_admin: bool = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);
}