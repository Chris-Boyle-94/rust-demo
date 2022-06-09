
use std::io;

fn main() {
    // Recursive HANOI function
    fn recur_hanoi((n, source, target, aux): 
        (u32, &mut Vec<u32>, &mut Vec<u32>, &mut Vec<u32>)) {   
        if n > 0 {
            println!("n\t{}\nsource\t{:?}\ntarget\t{:?}\naux\t{:?}\n", n, source, target, aux);
            recur_hanoi((n-1, source, aux, target));
            target.push(source.remove(source.len() - 1));
            println!("n\t{}\nsource\t{:?}\ntarget\t{:?}\naux\t{:?}\n", n, source, target, aux);
            recur_hanoi((n-1, aux, target, source));
        }
    }
    
    println!("Enter number of disks to move: ");
	let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    let input: u32 = match input.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => 0
    };
    let mut a: Vec<u32> = (1..input+1).rev().collect();
    let mut b: Vec<u32> = Vec::new();
    let mut c: Vec<u32> = Vec::new();
    println!("a\t{:?}", a);
    println!("b\t{:?}", b);
    println!("c\t{:?}\n", c);
    recur_hanoi((input, &mut a, &mut c, &mut b));
    println!("a\t{:?}", a);
    println!("b\t{:?}", b);
    println!("c\t{:?}", c);
}
