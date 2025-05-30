pub fn print_booleans() {
     let a: u8 = 5;
    let b: u8 = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 > 10: {}", a > b);

    let logged_in: bool = true;
    let is_admin: bool = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    let can_vote: bool = 19 >= 16;

    println!("Can vote? {}", can_vote);

    if can_vote {
        println!("You can vote");
    } else {
        println!("You cannot vote");
    }
}