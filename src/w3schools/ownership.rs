pub fn print_ownership() {
    println!("\nThis is the ownership.rs file****");
    //Ownership for strings, the value gest moved not copied
    let a:String = String::from("Hello");
    let b:String = a;

    println!("{}", b);
    //println!("{}", a); Gives borrowed value of a error

    //For numbers, characters and booleans the values are copied not moved so you can use the original value still
    let c:u8 = 10;
    let d:u8 = c;

    println!("{}", c);
    println!("{}", d);

    //.clone() to create a copy instead of move values for strings
    let e:String = String::from("Hello");
    let f:String = e.clone();

    println!("{}", e);// works
    println!("{}", f);// works
}