use std::io;

fn main() {

   println!("Good Day!: ");

   loop {

   let mut researcher = String::new();
   println!("What researcher are you, \n Enter your number in digits: "); 
   io::stdin().read_line(&mut researcher).expect("Failed to read input");
   let researcher:f32 = researcher.trim().parse().expect("Failed to read input");

   let mut name = String::new();
   println!("Please Enter your name: ");
   io::stdin().read_line(&mut name).expect("Failed to read input");

   let mut no_of_papers = String::new();
   println!("Please Enter your number of paper: ");
   io::stdin().read_line(&mut no_of_papers).expect("Failed to read input");
   let n:f32 = no_of_papers.trim().parse().expect("Failed to read input");

   let i1:f32 = 500_000.0;
   let i2:f32 = 800_000.0;
   let i3:f32 = 1_000_000.0;
   let i4:f32 = 100_000.0;

   if n >= 3.0 && n <= 5.0
   {
      println!("Your name is {}", name);println!("Your incentive is {}",i1);
   }
   else if n > 5.0 && n < 10.0 
   {
      println!("Your name is {}", name);println!("Your incentive is {}", i2);
   }
   else if n > 10.0
   {
      println!("Your name is {}", name);println!("Your incentive is {}", i3);
   }
   else if n < 3.0
   {
      println!("Your name is {}", name);println!("Your incentive is {}",i4); 
   }
   if researcher == 500.0 {
      break;
   }
   }
}
