use std::io;

fn main() {
    'main: loop {
        //get input from user
        let mut input=String::new();
        println!("Please enter a number: ");
        io::stdin().read_line(&mut input).expect("Not a string");
        let num_str = input.trim(); //trim whitespace and save input

        //check if input is valid (must be an integer)
        if !num_str.parse::<i64>().is_ok() {
            println!("That was not an integer number! Please try again. ");
            continue;
        }

        //check if number is an Armstrong number
        let num: u32 = num_str.parse().unwrap();
        let digit_count = num_str.chars().count() as u32;

        let mut sum: u32 = 0;
        for nth_char in num_str.chars() {
            let nth_digit: u32 = nth_char as u32 - '0' as u32;
            sum += nth_digit.pow(digit_count);
        }

        //check if the "armstrong sum" is equal to the number entered
        if sum == num {
            println!("That IS an Armstrong number! ")
        } else {
            println!("That is NOT an Armstrong number. ")
        }

        //ask user if they would like to enter another number
        'another: loop {
            let mut input = String::new();
            println!("Would you like to check another number? (Y/N) ");
            io::stdin().read_line(&mut input).expect("Not a string");
            let answer = input.trim();
            if answer == "Y" || answer == "y" || answer == "Yes" || answer == "yes" {
                continue 'main;
            } else if answer == "N" || answer == "n" || answer == "No" || answer == "no" {
                println!("Goodbye! ");
                break 'main;
            } else {
                println!("Not sure what you meant... ");
                continue 'another;
            }
        }
    }
}