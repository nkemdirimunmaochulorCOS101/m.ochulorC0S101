use std::io;

fn main()
{
    let mut d1 = String::new();
    let mut d2 = String::new();
    let mut t1 = String::new();
    let mut t2 = String::new();

    println!("Enter the first distance of the car");
    io::stdin().read_line(&mut d1).expect("Not a valid string");
    let d1:f32 = d1.trim().parse().expect("Not a valid number");

    println!("Enter the fisrt time carried out by the car");
    io::stdin().read_line(&mut t1).expect("Not a valid string");
    let t1:f32 = t1.trim().parse().expect("Not a valid number");

    let distance1 = d1 * 1.609;
    let speed1:f32 = distance1 / t1;
    println!("Speed of the car = {}", speed1);

    println!("Enter the second distance of the car");
    io::stdin().read_line(&mut d2).expect("Not a valid string");
    let d2:f32 = d2.trim().parse().expect("Not a valid number");

    println!("Enter the second time carried out by the car");
    io::stdin().read_line(&mut t2).expect("Not a valid string");
    let t2:f32 = t2.trim().parse().expect("Not a valid number");

    let distance2 = d2 * 1.609;
    let speed2 = distance2 / t2;
    println!("Speed of the car = {}", speed2);


}

