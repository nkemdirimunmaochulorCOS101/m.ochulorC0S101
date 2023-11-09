use std::io;

fn main() {
    println!("    MENU                     Price 

        p = Poundo Yam/ Edinkaiko Soup      3_200.0
        f = Fried rice & Chicken            3_000.0
        a = Amala & Ewedu Soup              2_500.0
        e = Eba & Egusi Soup                2_000.0
        w = White Rice & Stew               2_500.0


        ENJOY YOUR MEAL!
"); 

    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Enter your desired meal:) : ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let input1 = input1.trim();

    println!("Enter the the quantity: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let j:f32 = input2.trim().parse().expect("Not a valid number");

    let p:f32 = 3200.0;
    let f:f32 = 3000.0;
    let a:f32 = 2500.0;
    let e:f32 = 2000.0;
    let w:f32 = 2500.0;
    let mut g:f32 = 0.0;

    if input1 == "p" {
    let t = j * p;

        println!("Total charges is {}",t);
        g += t;
    } 
    else if  input1 == "f"{ 
    let t = j * f;

        println!("Total charges is {}",t);
        g += t;
    } 
    else if  input1 == "a"{
    let t = j * a;

        println!("Total charges is {}",t);
        g += t;
    } 
    else if  input1 == "e" {
    let t = j * e;
    
        println!("Total charges is {}",t);
        g += t;
    }
    else if input1 == "w"{
    let t = j * w;
        println!("Total charges is {}",t);
        g += t;
    }
    if g > 10_000.0 {
    let n = g - (0.05 * g);
    
       println!("The discount of your total charges is {} HURRAY!!!",n ); 
    }
    else if g <= 10_000.00
    {
        println!("No discount for youuu!!");
    }
}
