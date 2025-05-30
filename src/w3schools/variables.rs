pub fn print_variables() {
    println!("This is the variable.rs file****");
    /* You can add multi line comments
     Here */
    //This will output Hello World and add an end line after World
    print!("Hello");
    print!(" World\n"); //Can also add Comments here
    let name = "John";
    println!("My first name is: {}", name);
    let age = 30;

    println!("{} is {} years old", name, age);
    
    //let x = 5; You cannot change an immutable variable, all data is immutable by default
    let mut x = 5; //You add the mut keyword to make it mutable
    println!("x before: {}", x);
    x = 10;
    println!("x after: {}", x);
}