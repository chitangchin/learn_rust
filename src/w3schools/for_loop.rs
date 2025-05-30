pub fn print_for_loop() {
    println!("\nThis is the for_loop.rs file****");

    //-10 (inclusive) => 4 (exclusive)
    for number in (-3..4i8).rev() {
        println!("{}", number);
    }

    println!("\n");

    //0 (inclusive) => 4 (inclusive)
    for number in 0..=4u8 {
        println!("i is: {}", number);
    }

    println!("\n");

    //0 (inclusive) => 5 (exclusive) 6 (inclusive) => 7 (exclusive)
    for number in 0..10u8 {
        if number == 5 {
            continue;
        }
        if number == 7 {
            break;
        }
        println!("i is: {}", number);
    }
}