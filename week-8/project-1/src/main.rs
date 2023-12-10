use std::io;


fn main() {

    println!("GOOD DAY, \nYOUR ARE WELCOME TO THE PUBLIC SERVICE APS LEVEL CHECKER FOR THE FEDERAL GOVERNMENT OF NIGERIA");
    let mut j = String::new();
    println!("What job do you hold: ");
    io::stdin().read_line(&mut j).expect("Not a valid string");
    let job = j.trim().to_uppercase();

    let mut first_name = String::new();
    println!("Enter your first name: ");
    io::stdin().read_line(&mut first_name).expect("Not a valid string");
    let first_name = first_name.trim().to_uppercase();

    let mut last_name = String::new();
    println!("Enter your last name: ");
    io::stdin().read_line(&mut last_name).expect("Not a valid string");
    let last_name = last_name.trim().to_uppercase();

    let mut age = String::new();
    println!("Enter you age: ");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age = age.trim().to_uppercase();

    let mut years_of_experience = String::new();
    println!("Enter your years of experience: ");
    io::stdin().read_line(&mut years_of_experience).expect("Not a valid string");
    let yre = years_of_experience.trim().to_uppercase();

    let mut dob = String::new();
    println!("Enter your date of birth: ");
    io::stdin().read_line(&mut dob).expect("Not a valid string");
    let dob = dob.trim().to_uppercase();

    let years_of_experience = String::new();
    println!("Enter your years of experience: ");
    io::stdin().read_line(&mut years_of_experience).expect("Not a valid string");
    let years_of_experience = years_of_experience.trim().to_uppercase();


 if let  j = "O" {
 office_administrator();
 }
 else if let j = "A" {
 academic();
 }
 else if let j = "L" {
 lawyer();
 }
 else if let j = "T" {
 teacher();
 }
 else {
     println!("invalid input");
 }
}

fn office_administrator() {
    let office : Vec<&str> = vec!["Intern", "Administrator", "Senior Administraror", "Office Manager", "Director", "CEO"];
     let mut index = String::new();
     println!("What position do you hold in this field between (0 - 5: ");
     io::stdin().read_line(&mut index).expect("Failed to read input");
     let index: <&str> :: index::trim();

     if index == office[0] && years_of_experience{
        println!("the staff hilds posiyion  APS 5-8");
     }
 }

fn academic() {

    let a = vec!["Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
}
fn lawyer() {

    let l = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
}
fn teacher() {

    let t = vec!["Placement", "Classroon Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
}   