pub fn print_rust_string() {
    println!("\nThis is the rust_string.rs file****");
    //Two ways to create strings
    // let text1 = "Hello World".to_string();
    // let text2 = String::from("Hello World");

    //Strings are muttable when declared with mut
    let mut greeting: String = String::from("Hello");

    //Use push_str to add a text to the string
    greeting.push_str(" World");
    println!("{}", greeting);

    //Use push to add a character to the string
    greeting.push('!');
    println!("{}", greeting);

    //Adding two strings
    let s1: String = String::from("Hello");
    let s2: String = String::from("World!");
    let s3: String = String::from("What a beautiful day!");

    let result: String = format!("{} {} {}", s1, s2, s3);
    println!("{}", result);

    let result2: String = s1 + " " + &s2 + " " + &s3;
    println!("{}", result2);

    //You can only add a &str to a String with + so thats why we use the & reference pointer
    let a1: String = String::from("Hello");
    let b2: &str = "Brandon";
    let c3: &str = "!";
    let result3: String = a1 + " " + b2 + c3;
    println!("{}", result3);

    let name: String = String::from("John");
    println!("Length: {}", name.len()); // 4
}