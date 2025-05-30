use std::cmp::Ordering;

fn dataTypes() {
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

}