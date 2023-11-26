use std::io;

fn main() {
   let mut input1 = String::new();
   println!("Enter number of siblings: ");
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let no = input1.trim().parse().expect("Invalid input");

   for x in 1..=no{
   let mut siblings = String::new();
   println!("Enter your siblings name: ");
   io::stdin().read_line(&mut siblings).expect("Failed to read input");
   let s = siblings.trim().to_lowercase();

   let mut age = String::new();
   println!("Enter your siblings age: ");
   io::stdin().read_line(&mut age).expect("Failed to read input");
   let a:f32 = age.trim().parse().expect("Invalid input");
  
    if a > 18.0{
     println!("Is the sibling married or single: ");
     let mut  marital_status = String::new();
   io::stdin().read_line(&mut marital_status).expect("Failed to read input");
   let m = marital_status.trim().to_lowercase();  
    
    if m == "single"{

     println!("Is the sibling a student or worker: ");
     // assing k to the string if the the sibling is a student or worker
   let mut input2 = String::new();
   io::stdin().read_line(&mut input2).expect("Failed to read input");
   let k = input2.trim().to_lowercase();
    
    if  k == "student"{

     println!("Enter current university: ");
      let mut university = String::new();
   io::stdin().read_line(&mut university).expect("Failed to read input");
   let u = university.trim().to_lowercase();

   println!("Enter course of study: ");
   let mut course_of_study = String::new();
   io::stdin().read_line(&mut course_of_study).expect("Failed to read input");
   let c = course_of_study.trim().to_lowercase();  

    }}
    

    if m == "married"{
    
     println!("Does the sibling have any children: ");
     // asigning if the sibling has children to input3
   let mut input3 = String::new();
   io::stdin().read_line(&mut input3).expect("Failed to read input");
   let y = input3.trim().to_lowercase();  

   println!("What city does the sibling live in with their family: ");
    let mut city = String::new();
   io::stdin().read_line(&mut city).expect("Failed to read input");
   let t = city.trim().to_lowercase(); 
    }



    if a < 18.0{
    
     println!("Enter the siblings WAEC status: ");
     let mut status = String::new();
     io::stdin().read_line(&mut status).expect("failed to read input");
     let status = status.trim().to_lowercase();
    
    if status == "yes"{
    println!("Enter secondary school attended: ");
    let mut sec_school = String::new();
   io::stdin().read_line(&mut sec_school).expect("Failed to read input");
   let sec_school = sec_school.trim().to_lowercase(); 

    }
    else if status == "no"{
    
     println!("Enter class level");
     let mut class_level = String::new();
   io::stdin().read_line(&mut class_level).expect("Failed to read input");
   let j = class_level.trim().to_lowercase(); 
    }

    else{
     println!("Cannot continue with this data coolection");
    }
    }
}
}


