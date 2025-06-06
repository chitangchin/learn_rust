//If you want to use a value without taking ownership of it, you can use reference (borrowing)
pub fn print_borrowing_reference() {
    println!("\nThis is the borrowing_reference.rs file****");
    let a = String::from("Hello");
    let b = &a;
    println!("a = {}", a);
    println!("b = {}", b);
    //b is borrowing value from a not owning it

    //To change a value through a reference you need to make the reference mut
    let mut name = String::from("Brandon");
    let name_ref = &mut name;
    name_ref.push_str(" Chin");

    println!("{}", name_ref);
}
