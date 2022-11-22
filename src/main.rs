use std::io;

fn main() {

    const NUMBER_OF_CHOICES: i32 = 4;

    loop {

        println!("\n--==Menu==--");
        println!("1. Farhenheit to Celsius");
        println!("2. Celsuis to Farhenheit");
        println!("3. Fibonacci");
        println!("4. Twelve days of Christmas");
        println!("------------------------");
        println!("0. Quit");
        println!("Your choice :");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let choice: i32 = user_input.trim().parse().expect("Failed to convert to i32");

        if choice == 0 {
            break;
        } else if choice > NUMBER_OF_CHOICES {
            continue;
        } else if choice == 4 {
            twelve_days_of_christmas();
        } else {

            println!("Your input: ");

            let mut user_input = String::new();

            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read input");
            
            let user_data: i32 = user_input.trim().parse().expect("Failed to convert to i32");

            let result:i32;

            if choice == 1 {
                result = fahrenheiet_to_celsius(user_data);
            } else if choice == 2 {
                result = celsius_to_fahrenheit(user_data);
            } else if choice == 3 {
                result = fibonacci(user_data);
            } else {
                continue;
            }
            
            println!("Result is: {}", result);
        }
    }

    println!("Goodbye!");
}

fn fahrenheiet_to_celsius(fahrenheit: i32) -> i32 {
    ((fahrenheit - 32) * 5)/9
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    ((celsius * 9)/5) + 32
}

fn fibonacci(limit: i32) -> i32{
    
    let mut term_1 = 0;
    let mut term_2 = term_1 + 1;
    let mut term_3 = 0;
    for _i in 0..limit {
        term_3 = term_1 + term_2;
        term_1 = term_2;
        term_2 = term_3;
    }
    term_3
}

fn twelve_days_of_christmas() {
    let lyrics_stacks = 
    ["Twelve drummers drumming", "Eleven pipers piping", "Ten lords a-leaping",
    "Nine ladies dancing", "Eight maids a-milking", "Seven swans a-swimming",
    "Six geese a-laying", "Five golden rings", "Four calling birds",
    "Three french hens", "Two turtle doves, and", "A patridge in a pear tree"];

    let numbers = ["first", "second", "third", "fourth", "fifth",
    "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let lyrics_stacks_len: usize = lyrics_stacks.len();

    for i in 1..lyrics_stacks_len+1 {
        println!("On the {} day of Christmas, my true love sent to me", numbers[i-1]);
        for j in 0..i {
            println!("{}", lyrics_stacks[lyrics_stacks_len-i + j]);
        }
        println!("");
    }

}
