use std::io;

fn main() {
    println!("Enter your value for n: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let input1:i32 = input1.trim().parse().expect("Not a valid number");
    
    for lhs in 1..=input1 {
        for rhs in 1..={
            let result = lhs * rhs;
            println!("{} x {} = {}", lhs, rhs, result);
        }
        println!();
    } 

}

