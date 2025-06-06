pub fn print_scope() {
    println!("\nThis is the scope.rs file****");
    scoped_variable();
}

fn scoped_variable() {
    let mut result = "Pass"; //mut keyword to allow it to adjust
    println!("Result: {}", result);
    if result == "Pass" {
        let random_int = 10;
        println!("{}", random_int);
        result = "No Pass";
    }
    //println!("{}", random_int) <- will cause error because variable is outside of scope where it was intiated
    println!("{}", result); // will give No Pass and work since it is in scope of where variable was initialized

    let x:u8 = 5;
    {
        let x:u8 = 10;
        println!("Inside Block: {}", x); // 10
    }
    println!("Outside Block: {}", x); // 5

    //let x = 5;
    //let x = 10; This wouldnt work since they are in the same scope
}