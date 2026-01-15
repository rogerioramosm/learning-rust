fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("THREE_HOURS_IN_SECONDS = {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value os x in ther inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces: {spaces}");

    println!("\n==========================================================================================================\n");

    let _guess: i32 = "-42".parse().expect("Not a number!");

    println!("_guess: {}", _guess);

    println!("\n==========================================================================================================\n");

    let number: i32 = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("\n==========================================================================================================\n");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    println!("\n==========================================================================================================\n");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // loop infinito
    /*loop {
        println!("again!");
    }*/

    println!("\n==========================================================================================================\n");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    println!("\n==========================================================================================================\n");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    println!("\n==========================================================================================================\n");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("\n==========================================================================================================\n");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is : {}", a[index]);

        index += 1;
    }

    println!("\n==========================================================================================================\n");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!("\n==========================================================================================================\n");

    for number in (1..5).rev() {
        println!("{number}");
    }

    println!("LEFTOFF!!!");

    println!("\n==========================================================================================================\n");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!(
        "five_hundred ={}, six_point_four = {}, one = {},",
        five_hundred, six_point_four, one
    );
}
