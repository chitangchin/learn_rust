pub fn print_loops() {
    
    let mut x: u8 = 0;
    let result:u8 = loop {
        if x > 5 {
            break x;
        }
        println!("{}", x);
        x += 1;
    };

    println!("The result is: {}", result);
    // Statements end if ;, expressions dont have ;
}