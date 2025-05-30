use std::cmp::Ordering;

fn function() {
    //Variables are immutable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    //Constants cannot have the mut tag, and data type must be notated
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; <- gives unused variable error

    //Shadowing variables
    let y = 10;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); //22
    }

    println!("The value of y is: {y}"); // 11

    // let spaces = "   "; //str
    // let spaces = spaces.len(); // num, we can save space instead of creating new variables for the string and number variable by reusing let

    /*
    Cannot do this with mut:
    let mut spaces = "   ";
    spaces = spaces.len()

    gives "expected &str, found usize"
    */
    //Array

    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let b = [3; 10];
    println!("first: {}", a[0]); //1
    println!("second: {}", b[3]); //3

    //Accessing an invalid element falls in a runtine error handler instead of accessing invalid memory. Memory safety principle 
    let mut count = 0;
    loop {
        match count.cmp(&5) {
            Ordering::Equal => break,
            Ordering::Less => 
            {
                println!("{}",add(a[count], b[count]));
                num(a[count]);
                count += 1;
            },
            Ordering::Greater => count += 1,
        }
    }
    //You can have if expressions inside of statements, if else must return the same data type
    let number = if a[0] == 1 { 5 } else { 6 };
    num(number);

    let mut counter = 0;

    while counter != 10 {
        println!("{counter}");
        counter += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for nums in (1..10).rev() {
        println!("{nums}!");
        num(nums);
    }

}

fn add(x: u8, y: u8) -> u8{
    println!("{}", x + y);
    x + y - 1 // expressions dont end with semicolon, statements end with semicolon, you cannot return a statement because statements dont have return value
}

fn num(x: u8) {
    //The second if statment doenst execute, it stops at the first truthy condition in the if expression
    if x == 5 {
        println!("number is {}", x);
    } else if x % 5 == 0 {
        println!("This is also true if its 5");
    } else {
        println!("number is not 5!");
    }
}