use std::io;
mod fib;
mod hanoi;

fn main() {
    loop {
        println!("\nSelect a program to run:");
        println!("\t1: Temp converter");
        println!("\t2: Fibo Finder");
        println!("\t3: Twelve Days of Christmas");
        println!("\t4: Tower of Hanoi");
        println!("\tq: Quit\n");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("There was an error here for some reason");

        match input.trim() {
            "1" => temp_converter(),
            "2" => fib::main(),
            "3" => twelve_days(),
            "4" => hanoi::main(),
            "q" | "Q" => break,
            _ => {
                println!("\nPlease enter a listed option\n");
                continue;
            }
        };

        let another: bool = loop {
            println!("\nWould you like to run another program? (y/n)\n");

            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Something went wrong, dawg.");

            let input: bool = match input.trim().to_lowercase().as_str() {
                "y" => true,
                "n" => false,
                _ => {
                    println!("\nPlease enter y or n\n");
                    continue;
                }
            };

            break input;
        };

        if another == false {
            println!("\nThanks for checking out my programs!\n");
            break;
        }
    }
    moot() // see definition at bottom of file
}

// Converts temperature between Fahrenheit and Celsius and prints result
fn temp_converter() {
    println!("\nCONVERT TEMPERATURES (F -> C or C -> F)\n");

    let selected_type: String = loop {
        println!("Choose a temp type:");
        println!("\t1. Fahrenheit");
        println!("\t2. Celsius");

        let mut temp_type: String = String::new();

        io::stdin()
            .read_line(&mut temp_type)
            .expect("Something went wrong...");

        let temp_type: String = match temp_type.trim() {
            "1" => String::from("Fahrenheit"),
            "2" => String::from("Celsius"),
            _ => {
                println!("\nPlease enter either '1' or '2'\n");
                continue;
            }
        };

        break temp_type;
    };

    let converted_type: String = if selected_type == "Fahrenheit" {
        String::from("Celsius")
    } else {
        String::from("Fahrenheit")
    };

    println!("\n{} -> {}", selected_type, converted_type);

    let selected_temp: f32 = loop {
        println!("\nPlease enter a temperature to convert\n");

        let mut temp: String = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Something went wrong, dude..");

        let temp: f32 = match temp.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a number\n");
                continue;
            }
        };
        break temp;
    };

    let converted_temp: f32 = if selected_type == "Fahrenheit" {
        let sub: f32 = selected_temp - 32.0;
        sub / 1.8
    } else {
        let mult: f32 = selected_temp * 1.8;
        mult + 32.0
    };

    println!(
        "\nYour converted temperature is: {} degress {}\n",
        converted_temp, converted_type
    );
}

/** fibo
 * Prints the nth fib number
 * 
 * fn fibo() {
 *     println!("FIBO FINDER");
 * 
 *     let position: u64 = loop {
 *         println!("\nPlease enter the position of the number you would like to find (i.e. position 10 is fib number 55)\n");
 * 
 *         let mut input: String = String::new();
 * 
 *         io::stdin()
 *             .read_line(&mut input)
 *             .expect("Something went wrong");
 * 
 *         let input: u64 = match input.trim().parse::<u64>() {
 *             Ok(num) => num,
 *             Err(_) => continue,
 *         };
 * 
 *         if input > 0 && input < 94 {
 *             break input;
 *         } else {
 *             println!("\nPlease enter a number 1-93.\n");
 *             continue;
 *         }
 *     };
 * 
 *     let fib_number: u64 = if position == 1 {
 *         1
 *     } else {
 *         let mut count: u64 = 1;
 *         let mut prev: u64 = 0;
 *         let mut current: u64 = 1;
 *         loop {
 *             let next: u64 = current + prev;
 *             prev = current;
 *             current = next;
 *             count += 1;
 * 
 *             if count == position {
 *                 break current;
 *             }
 *         }
 *     };
 * 
 *     println!(
 *         "\nThe fibonacci number at position {} is: {}\n",
 *         position, fib_number
 *     );
 * }
 */


// Prints the lyrics to The Twelve Days of Christmas
fn twelve_days() {
    println!("\nTHE TWELVE DAYS OF CHRISTMAS\n");

    let days_arr: [&str; 12] = [
        "a partridge in a pear tree\n",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five gold rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven piper piping,",
        "twelve drummers drumming,",
    ];

    for i in 1..13 {
        let day: String = match i {
            1 => String::from("1st"),
            2 => String::from("2nd"),
            3 => String::from("3rd"),
            _ => format!("{}th", i),
        };

        println!("On the {} day of Christmas my true love gave to me:", day);

        for j in (0..i).rev() {
            if j != 0 || i == 1 {
                println!("\t{}", days_arr[j]);
            } else {
                println!("\tand {}", days_arr[j])
            }
        }
    }
}

/** tower
 * fn tower() {
 *     // With 3 disks, the puzzle can be solved in 7 moves.
 *     // The minimal number of moves required to solve a Tower of Hanoi puzzle is 2n − 1,
 *     // where n is the number of disks.
 * 
 *     let mut first: Vec<u8> = Vec::new();
 *     let mut second: Vec<u8> = Vec::new();
 *     let mut third: Vec<u8> = Vec::new();
 * 
 *     let stack_size: u8 = loop {
 *         let mut input = String::new();
 * 
 *         println!("Stack size:\n");
 * 
 *         io::stdin().read_line(&mut input).expect("ERROR ERROR");
 * 
 *         let input = match input.trim().parse::<u8>() {
 *             Ok(num) => num,
 *             Err(_) => continue,
 *         };
 * 
 *         if input > 0 || input < 255 {
 *             break input - 1;
 *         } else {
 *             println!("Please enter a number 0-256");
 *             continue;
 *         }
 *     };
 * 
 *     for i in 0..stack_size {
 *         first.push(i + 1);
 *     }
 * 
 *     for i in 0..10 {
 *         println!("{}", first[i])
 *     }
 * }
 * 
 */

fn moot() {
 // lazy placeholder so I can fold unused functions using doc comments
}