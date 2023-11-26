use std::io;

fn main () 
{
    println!(" 
                      HEALTH DIAGONOSIS              AMOUNT      VILLAGE    DISCOUNT
                       k = Alzhelmer                1_200_000   Akpabom       20%
                       r = Arrhythmia               550_000     Ngbauji       5%
                       u = Chronic Kidney Disease   1_500_000   Atabrinkang   15%
                       d = Diabetes                 800_000     Okorobilom    10%
                       h = Arthritis                450_000     Emeremen      10%  ");


    loop {
    
    let mut user = String::new();
    println!("Enter your user number: ");
    io::stdin().read_line(&mut user).expect("Not a valid string");
    let user:f32 = user.trim().parse().expect("Not a valid number");

    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Not a valid string");

    let mut place_of_birth = String::new();
    println!("Enter your date of birth: ");
    io::stdin().read_line(&mut place_of_birth).expect("Not a valid string");

    let mut age = String::new();
    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let a:f32 = age.trim().parse().expect("Not a valid number");

    let mut email_address = String::new();
    println!("Enter your email address: ");
    io::stdin().read_line(&mut email_address).expect("Not a valid string");

    let mut phone_number = String::new();
    println!("Enter your phone number: ");
    io::stdin().read_line(&mut phone_number).expect("Not a valid string");

    let mut no_of_siblings = String::new();
    println!("Enter your number of siblings: ");
    io::stdin().read_line(&mut no_of_siblings).expect("Not a valid string");
    let n:f32 = no_of_siblings.trim().parse().expect("Not a valid number");

    let mut no_of_children = String::new();
    println!("Enter your number of children: ");
    io::stdin().read_line(&mut no_of_children).expect("Not a valid string");
    let f:f32 = no_of_children.trim().parse().expect("Not a valid string");

    let mut medical_diagonosis = String::new();
    println!("Enter your medical diagonosis: ");
    io::stdin().read_line(&mut medical_diagonosis).expect("Not a valid string");

    let mut village_of_residence = String::new();
    println!("Enter your village of residence: ");
    io::stdin().read_line(&mut village_of_residence).expect("Not a valid string");

    let p:f32 = 1_200_000.0;
    let r:f32 = 550_000.0;
    let c:f32 = 1_500_000.0;
    let d:f32 = 800_000.0;
    let s:f32 = 450_000.0;

    let v = "Akpabom";
    let j = "Ngbauji";
    let m = "Atabrinkang";
    let y = "Okorobiolom";
    let b = "Emeremen";
    
    let dis1:f32 = 1_200_000.0 * 0.20;
    if medical_diagonosis == "k" && a > 50.0 && f == 4.0 && village_of_residence == "v"
    {
        println!("Your medical diagonosisis is {}", k);
        println!("Your village of residence is {}", v);
        println!("You are {} years of age ", a);
        println!("You have more than 4 children");
        println!("Your discount is {}", dis1);
    }  
    else  
    {
        println!("the total charge is {}", p);
    }
    let dis2:f32 = 550_000.0 * 0.05;
    if medical_diagonosis == "r" && a == 30.0 && n > 4.0 && village_of_residence == "j"
    {
        println!("Your medical diagonosis is {}", r);
        println!("Village of residence is {}", j);
        println!("You are 30 years old");
        println!("You have 4 siblings"); 
        println!("Your discount is {}", dis2);
    } 
    else
    {
        println!("the total charge is {}", r);
    }
    let dis3:f32 = 1_500_000.0 * 0.15;
    if medical_diagonosis == "u" && a > 40.0  &&  n > 3.0  && f > 3.0 && village_of_residence == "m"
    {
        println!("Your medical diagonosis is {}", u);
        println!("Village of residence is {}", m);
        println!("You are {} years of age", a); 
        println!("You have more than 3 children an siblings"); 
        println!("Your discount is {}", dis3);
    }
    else 
    {
         println!("the total charge is {}", c);
    } 
    let dis4:f32 = 800_000.0 * 0.1;
    if medical_diagonosis == "d" && a > 28.0 && a < 45.0 && f > 2.0 && f < 4.0 && village_of_residence == "y"
    {
        println!("Your medical diagonosis is {}", d);
        println!("Village of residence is {}", y);
        println!("Your are {} years of age", a);
        println!("You have {} children", f);
        println!("Your discount is {}", dis4);
    }
    else 
    {
        println!("the total charge is {}", d);
    }
    let dis5:f32 = 450_000.0 * 0.1;
    if medical_diagonosis == "h" && a > 58.0 && n > 5.0 && f > 5.0 && village_of_residence == "b"
    {
        println!("Your medical diagonosis is {}", h);
        println!("Village of residence is {}", b);
        println!("Your are {} years of age", a);
        println!("You have {} children", f);
        println!("Yoy have {} no_of_siblings", n);
        println!("Your discount is {}", dis4);
    }
    else
    {
        println!("the total charge is {}", s);
    }
    if user == 100.0 {
            break;
    }
    }
}
  