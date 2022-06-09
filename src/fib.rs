
use std::io;

fn main() {
    fn quick_fibo(depth:u32) -> u64{
        if depth == 0 {
            return 0;
        } else if depth == 1 || depth == 2 {
            return 1;
        } else {
            return recur_fibo((depth-1, 1, 1));
        }
    }
    // Recursive FIB function
    fn recur_fibo((depth, _val1, _val2): (u32, u64, u64)) -> u64{   
        println!("{}", _val2);
        if depth > 1 {
            return recur_fibo((depth-1, _val2, _val1+_val2));
        } else {
            return _val2;
        }
    }
    
    println!("Enter Fib position to find: ");
	let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    let input: u32 = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => 0
    };

    let val: u64 = quick_fibo(input);
    println!("\nFib num of {}:\n\t{}\n", input, val);
}
