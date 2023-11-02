use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first, edge of triangle: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string ");
    let a:f32 = input1.trim().parse().expect("Not a valid number ");

    println!("Enter second edge of triangle: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter second ege of triangle: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let s:f32 = a + b + c;
    let t:f32 = s / 20.0;
    let mut area:f32 = t * (t - a) * (t - b) * (t - c);
    area = area.sqrt();

    println!("Area of a triangle: {}", area);
}
