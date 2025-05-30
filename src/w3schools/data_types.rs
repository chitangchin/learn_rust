pub fn print_data_types() {
    // let my_num: i32 = 5;
    // let my_double: f64 = 5.99;
    // let my_letter: char = 'D';
    // let my_bool: bool = true;
    // let my_text: &str = "Hello";
    println!("\nThis is the data_types.rs file****");
    
    //Integer
    let age: i32 = 52;
    println!("My age is {}", age);

    //Floating Point
    let price: f64 = 19.99;
    println!("Price is: ${}", price);

    //Character
    let my_grade: char = 'B';
    println!("{}", my_grade);

    //String
    let name: &str = "John";
    println!("My name is {}", name);

    //Bool
    let is_logged_in: bool = true;
    println!("User logged in?: {}", is_logged_in);
}