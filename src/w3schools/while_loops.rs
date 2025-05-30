pub fn print_while_loops() {
    println!("\nThis is the while_loop.rs file****");

    let mut count: u8 = 1;

    while count <= 15 {
        println!("count: {}", count);
        count += 1;
        if count == 5 {
            break;
        }
    }

    let mut num: u8 = 1;

    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }
        println!("Number: {}", num);
        num += 1;
    }
}