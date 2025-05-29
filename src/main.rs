use std::cmp::Ordering;

fn main() {
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

    //Data Types
    /*
    
    */

    //let guess = "42".parse().expect("Not a number!"); // type must be known at this point
    //Correct:
    let _guess: u32 = "42".parse().expect("Not a number!");

    //Scalar Types
    /*
        Integers
        Floating-point number
        Boolean
        Characters

        Scalar Types represents a single value
    */

    //Integer Types
    //i8 = 8bit signed integer range: -128 to 127 i8::MAX
    const EIGHT_BIT_SIGNED_INT_N: i8 = -128;
    const EIGHT_BIT_SIGNED_INT_P: i8 = 127;
    const EIGHT_BIT_SIGNED: i8 = EIGHT_BIT_SIGNED_INT_N + EIGHT_BIT_SIGNED_INT_P;
    println!("{EIGHT_BIT_SIGNED_INT_N} + {EIGHT_BIT_SIGNED_INT_P} = {EIGHT_BIT_SIGNED}");

    //u8 = 8bit unsigned integer range: 0 to 255
    const EIGHT_BIT_UNSIGNED_INT_N: u8 = 128;
    const EIGHT_BIT_UNSIGNED_INT_P: u8 = 127;
    const EIGHT_BIT_UNSIGNED: u8 = EIGHT_BIT_UNSIGNED_INT_N + EIGHT_BIT_UNSIGNED_INT_P;
    println!("{EIGHT_BIT_UNSIGNED_INT_N} + {EIGHT_BIT_UNSIGNED_INT_P} = {EIGHT_BIT_UNSIGNED}");

    //i16 = 16bit signed integer range: -32,768 to 32,767
    const ST_BIT_SIGNED_INT_N: i16 = -32768;
    const ST_BIT_SIGNED_INT_P: i16 = 32767;
    const ST_BIT_SIGNED: i16 = ST_BIT_SIGNED_INT_N + ST_BIT_SIGNED_INT_P;
    println!("{ST_BIT_SIGNED_INT_N} + {ST_BIT_SIGNED_INT_P} = {ST_BIT_SIGNED}");

    //u16 = 16bit unsigned integer range: 0 to 65,535
    const ST_BIT_UNSIGNED_INT_N: u16 = 32768;
    const ST_BIT_UNSIGNED_INT_P: u16 = 32767;
    const ST_BIT_UNSIGNED: u16 = ST_BIT_UNSIGNED_INT_N + ST_BIT_UNSIGNED_INT_P;
    println!("{ST_BIT_UNSIGNED_INT_N} + {ST_BIT_UNSIGNED_INT_P} = {ST_BIT_UNSIGNED}");

    /*
    max value: (2^n) - 1

    i32 = 32bit signed integer range: -2,147,483,648 to 2,147,483,647
    u32 = 32bit unsigned integer range: 0 to 4,294,967,295
    
    i64 = 64bit signed integer range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    u64 = 64bit unsigned integer range: 0 to 18,446,744,073,709,551,615

    i128 = 128bit signed integer range: -170141183460469231731687303715884105728 to 170141183460469231731687303715884105728
    u128 = 128bit usigned integer range: 0 to 340282366920938463463374607431768211456

    isize = architecture based signed interger range: 64 bit or 32 bit range depending on computer
    usize = architecture based usigned integer range: 64 bit or 32 bit range depending on computer
    */

    //Floating-Point Types
    /*
        f64
        f32
    */

    //Math Operations
    // let sum = 5 + 10;

    // let difference = 95.5 - 4.3;
    
    // let product = 4 * 30;

    // let quotient = 56.7 / 32.2;

    // let truncated = -5/3;

    // let remainder = 43 % 5;

    //Boolean type
    // let t = true;
    // let f: bool = false;

    //character types
    // let c = 'z';
    // let z: char = 'z';
    // let heart_eyed_cat = '😻'; 

    //Primitive compound types - compound group multiple values in one type
    //Tuple - grouping variety of types into one compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //We can destructure a tuple to get the individual values
    let (x, y, z) = tup;
    println!("x: {x}"); // 500
    println!("y: {y}"); // 6.4
    println!("z: {z}"); // 1

    //can reference the value using the index
    let tup_two = tup.1;
    println!("{tup_two}"); //6.4

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