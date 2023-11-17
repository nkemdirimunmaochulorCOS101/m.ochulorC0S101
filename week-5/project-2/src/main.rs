use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter if experienced or inexperienced: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let e = input1.trim();
    

    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    
    if e == "experienced" && age >= 40.0
    {
        println!("The incentive of the employee is 1_560_000.0");
    }
    else if e == "experienced" && age < 40.0 && age >= 30.0
    {
        println!("The incentive of the employee is 1_480_000.0");
    } 
    else if e ==  "experienced" && age < 28.0
    {
        println!("The incentive of the employee is 1_300_000.0 per month");
    } 
    else if e == "inexperienced"
    {
        println!("The incentive is 100_000.0");
    }
    if  e != "inexperienced".to_owned() + "experienced" 
    {
        println!("Sorry this is not a basis of qualification");
    }
}

