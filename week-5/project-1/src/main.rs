use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");
    
    let dis = (b * b) - 4.0 * a * c;
    let r = (-b - (dis.sqrt())) / (2.0 * a);
    let t = (-b + (dis.sqrt())) / (2.0 * a);
     
    let dis = (b * b) - 4.0 * a * c;
    
    if dis > 0.0 
    {
        println!("\nThe roots of the equation are {} and {}", r , t);
        println!("There are two distinct roots");
    }

    else if dis == 0.0 
    {
        println!("\nThe roots of the equation are {} and {}", r , t);
        println!("There is exactly one real root");
    }

    else if dis < 0.0 
    {
        println!("There are no real roots");

    }
    println!("The discriminant of the equation is {}", dis );

}


