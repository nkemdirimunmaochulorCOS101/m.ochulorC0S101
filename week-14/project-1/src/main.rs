use std::io::Read;
use std::io;

fn main(){
    let mut file1 = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents1 = String::new();
    file1.read_to_string(&mut contents1).unwrap();
    

    let mut file2  = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents2 = String::new();
    file2.read_to_string(&mut contents2).unwrap();
    
    
    let mut file3 = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents3 = String::new();
    file3.read_to_string(&mut contents3).unwrap();
    

    let mut file4 = std::fs::File::open("department_tb.sql").unwrap();
    let mut contents4 = String::new();
    file4.read_to_string(&mut contents4).unwrap();
    

    let mut file5 = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents5 = String::new();
    file5.read_to_string(&mut contents5).unwrap();
    

    let mut file6 = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents6 = String::new();
    file6.read_to_string(&mut contents6).unwrap();

    let mut user = String::new();
    println!("Enter your username:");
    io::stdin().read_line(&mut user).expect("Not a valid string");
    let u = user.trim();
    if u == "administrator"
    {
        print!("{}", contents6);
    }
    else if u == "project manager"
    {
        print!("{}", contents3);
    }
    else if u == "employee"
    {
        print!("{}", contents5);
    }
    else if u == "customer"
    {
        print!("{}", contents1);
    }
    else if u == "vendor"
    {
        print!("{}", contents2);
    }
    else
    {
        println!("Not a valid entry");
    }
}