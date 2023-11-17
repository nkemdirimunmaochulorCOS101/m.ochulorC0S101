use std::io;

fn main() {


    println!("Good day! We are testing a digital sample system that conducts the Student Council Voting for Pan-Atlantic University");
    loop {

    let mut user = String::new();
    println!("What user are you: "); 
    io::stdin().read_line(&mut user).expect("Failed to read input");
    let user:f32 = user.trim().parse().expect("Failed to read input");

    let mut faculty_name = String::new();
    println!("Enter your faculty name: ");
    io::stdin().read_line(&mut faculty_name).expect("Failed to read input");
    
    let mut class_rep = String::new();
    println!("Are you a class rep, \n Enter 1 for yes or 2 for no: ");
    io::stdin().read_line(&mut class_rep).expect("Failed to read input");
    let rep:i32 = class_rep.trim().parse().expect("Not a valid number");
    println!("Alright, input taken");
    
    let mut level = String::new();
    println!("Are you in 100 level:\n Enter 4 for yes and 5 for no: ");
    io::stdin().read_line(&mut level).expect("Failed to read input");
    let l:f32 = level.trim().parse().expect("Not a valid number");
    println!("Alright, input taken");

    let mut cgpa = String::new();
    println!("What is you cgpa: ");
    io::stdin().read_line(&mut cgpa).expect("Failed to read input");
    let cgpa:f32 = cgpa.trim().parse().expect("Not a valid number");
    println!("Alright, input taken");
    
    let mut first_name = String::new();
    println!("Please enter your first name: ");
    io::stdin().read_line(&mut first_name).expect("Failed to read input");

    let mut last_name = String::new();
    println!("Please enter your last name: ");
    io::stdin().read_line(&mut last_name).expect("Failed to read input");

    let mut email = String::new();
    println!("Please enter your email: ");
    io::stdin().read_line(&mut email).expect("Failed to read input");

    let mut department = String::new();
    println!("Enter your department: ");
    io::stdin().read_line(&mut department).expect("Failed to read input");

    let mut state_of_origin = String::new();
    println!("Enter you state of origin: ");
    io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");

    if rep == 1 && l == 4.0 && cgpa > 4.0
    {
        println!("You can vote");
    }
    else 
    {
        println!("Sorry, not eligible to vote");
    }

    if user == 100.0 {
        break;
    } 
    }   
}