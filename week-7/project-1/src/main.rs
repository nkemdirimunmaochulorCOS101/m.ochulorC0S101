use std::io;

fn main() {

   println!("  
               let t = Area of a trapezium;
               let r = Area of the rhombus formula;
               let p = Area of parallelogram formula;
               let f = Area of cube formula;
               let c = Volume of cylinder formula;
   ");
   
   // assigning each equation to a variable
   

   // assigning equations to a variable
   let mut equ = String::new();
   println!("select equation:");
   io::stdin().read_line(&mut equ).expect("Failed to read input");
   let equ = equ.trim().to_lowercase();
   
   if equ == "t" {
   let mut height1 = String::new(); 
   println!("Enter the value of the height: ");
   io::stdin().read_line(&mut height1).expect("Failed to read input");
   let h1:f32 = height1.trim().parse().expect("Invalid input");

   let mut base1 = String::new();
   println!("Enter the value of the base1: ");
   io::stdin().read_line(&mut base1).expect("Failed to read input");
   let b1:f32 = base1.trim().parse().expect("Invalid input");

   let mut base2 = String::new();
   println!("Enter the value of the base2: ");
   io::stdin().read_line(&mut base2).expect("Failed to read input");
   let b2:f32 = base2.trim().parse().expect("Invalid input");

   let a1:f32 = (h1 / 2.0) * (b1 + b2);
   
   // to find the area of a trapezium
   // let a1:f32 = (h1 / 2.0) * (b1 + b2);
   
   println!("Area of the trapezium is {}", a1);
   
}
   else if equ == "r" {
   let mut diagonal1 = String::new();
   println!("Enter the value of the first diagonal: ");
   io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
   let d1:f32 = diagonal1.trim().parse().expect("Invalid input");

   let mut diagonal2 = String::new();
   println!("Enter the value of the second diagonal: ");
   io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
   let d2:f32 = diagonal2.trim().parse().expect("Invalid input");
   // to find the area of a rhombus
   let a2:f32 = 0.5 * d1 * d2;
   {
    println!("Area of a rhombus is {}", a2);
   }
}
   else if equ == "p" { 
   let mut base = String::new();
   println!("Enter the value of the base: ");
   io::stdin().read_line(&mut base).expect("Failed to read input");
   let b:f32 = base.trim().parse().expect("Invalid input");

   let mut altitude = String::new();
   println!("Enter the value of the altitude: ");
   io::stdin().read_line(&mut altitude).expect("Failed to read input");
   let a:f32 = altitude.trim().parse().expect("Invalid input");

   let a3:f32 = b * a;
   {
    println!("Area of the parallelogram is {}", a3);
   }
}
   else if equ == "f"{
   let mut length = String::new();
   println!("Enter the value of the length of the side: ");
   io::stdin().read_line(&mut length).expect("Failed to read input");
   let l:f32 = length.trim().parse().expect("Invalid input");
   
   // to find area of a cube
   let a4:f32 = 6.0 * (l * l);
   {
    println!("Area of cube {}", a4);
   }
}
   else if equ == "c" {
   let mut radius = String::new();
   println!("Enter the value of the radius: ");
   io::stdin().read_line(&mut radius).expect("Failed to read input");
   let r:f32 = radius.trim().parse().expect("Invalid input");

   let mut height2 = String::new();
   println!("Enter the value of the height: ");
   io::stdin().read_line(&mut height2).expect("Failed to read input");
   let h2:f32 = height2.trim().parse().expect("Invalid input");
   
   // to find the volume of a cylinder
   let volume:f32 = (22.0 * 7.0) * (r * r) * h2;
   {
    println!("Volume of cylinder {}", volume);
   }

}
    else {
        println!("Invalid Input");
    }
}
