pub fn print_if_else() {
    println!("\nThis is the if_else.rs file****");

    //Simple if expression
    if 5 >= 5 {
        println!("This statement is true");
    }

    //Simple if else expression
    let is_admin: bool = false;

    if is_admin {
        println!("This user is admin");
    } else {
        println!("This user is not admin");
    }

    //Simple if else else if expression
    let arg: &str = "fizzbuzz";

    if arg == "fizzbuzz" {
        println!("fizzbuzz!");
    } else if arg == "fizz" {
        println!("fizz");
    } else {
        println!("buzz");
    }

    let x: bool = if 10 > 5 {
        true
    } else {
        false
    };

    println!("{}", x);
}