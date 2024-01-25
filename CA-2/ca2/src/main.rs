 use std::io::Write;
 use std::io;

 fn main() {

     let companys = vec!["\nCadbury Nigeria Plc", "Champion Breweries Plc", "Dangote Sugar Refinery Plc", "Flour Mills Nigeria Plc", "Nestle Nigeria Plc", "Unilever Nigeria Plc", "Honeywell Nigeria Plc", "Nigeria Breweries Plc", "", ""];
     let companys_shares = vec!["\n15_000_000", "25_000_000", "18_000_000", "32_000_000", "8_000_000", "37_000_000", "34_000_000", "30_000_000", "", ""];
     let companys_liabilities = vec!["\n5_000_000", "8_000_000", "10_000_000", "4_000_000", "1_500_000", "11_000_000", "9_000_000", "12_000_000", "", ""];
     let date = vec!["\n1965", "1974", "1970", "1960", "1961", "1923", "1906", "1946", "", ""];
     let percent_leverage = vec!["\n33.3%", "32.0%", "55.5%", "12.5%", "18.8", "29.7%", "26.5%", "40.0%", "", ""];

     let mut comp = std::fs::File::create("data.txt").expect("create failed");
     comp.write_all(b"\n");

     comp.write_all(b"company");
    
     for c in 0..companys.len() {

        comp.write_all(companys[c].as_bytes()).expect("Create failed");
        comp.write_all(b"\n").expect("Create failed");
     }  
    
     comp.write_all(b"  company shares");

     for l in 0..companys_shares.len() {

        comp.write_all(companys_shares[l].as_bytes()).expect("Create failed");
        comp.write_all(b"\n").expect("Create failed");
     }  

     comp.write_all(b"  company liabilities ");
    
     for j in 0..companys_liabilities.len() {

        comp.write_all(companys_liabilities[j].as_bytes()).expect("Create failed");
        comp.write_all(b"\n").expect("Create failed");
     }  

     comp.write_all(b"  Date established");
    
     for d in 0..date.len() {

        comp.write_all(date[d].as_bytes()).expect("Create failed");
        comp.write_all(b"\n").expect("Create failed");
     }

     comp.write_all(b"  Percentage leverage");

     for v in 0..date.len() {

        comp.write_all(percent_leverage[v].as_bytes()).expect("Create failed");
        comp.write_all(b"\n").expect("Create failed");
     }

     {
         println!("Data has been written");
     }

    println!("               COMPANY                       SHARES            LIABILITIES
              Cadbury Nigeria Plc            15,000,000         5,500,000
              Champion Breweries Plc         25,000,000         8,000,000
              Dangote Sugar Refinery Plc     18,000,000         10,000,000
              Flour Mills Nigeria Plc        32,000,000         4,000,000
              Nestle Nigeria Plc             8,000,000          1,500,000
              Unilever Nigeria Plc           37,000,000         11,000,000
              Honeywell Nigeria Plc          34,000,000         9,000,000
              Nigerian Breweries Plc         30,000,000         12,000,000
       ");

     let mut username = String::new();
     println!("Enter your username ::\nNote user name must be four letters of the companys name");
     io::stdin().read_line(&mut username).expect("Not a valid string");
    
     let mut password  = String::new();
     println!("Enter your user number ::\nNote must contain \n1. Letters between [a-z] \n2. Letters between [0-9] \n3. No uppercase letter \n4. No characters from [$#@] \n5. Minimum Length of username = 3 \n6. Maximum length of username = 8");
     io::stdin().read_line(&mut password).expect("Not a valid string");

     let mut companys_share = String::new();
     println!("Enter your company share:");
     io::stdin().read_line(&mut companys_share).expect("Not a valid string");
     let mut s:f32 = companys_share.trim().parse().expect("Not a valid number");

     let mut companys_liabiliti = String::new();
     println!("Enter your company liability:");
     io::stdin().read_line(&mut companys_liabiliti).expect("Not a valid string");
     let mut k:f32 = companys_liabiliti.trim().parse().expect("Not a valid number");

     {
       println!("Username is {}", username);
       println!("Password is {}", password);
       println!("Company share is {}", s);
       println!("Companys liability is {}", k);
    }


      if s > 20_000_000.0 {
    
      let mut info = std::fs::File::create("info.txt").expect("Create failed");
      info.write_all(b"\n"); 

        info.write_all(b"company with shares greater than 20,000,000\n");
        {
         info.write_all(companys[1].as_bytes()).expect("Create failed");
         info.write_all(companys[3].as_bytes()).expect("Craete failed");
         info.write_all(companys[5].as_bytes()).expect("Create failed");
         info.write_all(companys[6].as_bytes()).expect("Create failed");
         info.write_all(companys[7].as_bytes()).expect("Create failed");
         info.write_all(b"\n");
      }
      info.write_all(b"The percentage leverage of companys with shares greater than 20,000,000\n");
      {
        info.write_all(percent_leverage[1].as_bytes()).expect("Create failed");
        info.write_all(percent_leverage[3].as_bytes()).expect("Craete failed");
        info.write_all(percent_leverage[5].as_bytes()).expect("Create failed");
        info.write_all(percent_leverage[6].as_bytes()).expect("Create failed");
        info.write_all(percent_leverage[7].as_bytes()).expect("Create failed");
        info.write_all(b"\n");
      
     } }
    {
        println!("data is written");
    }
 }
    struct Percentageleverage {
    cadbury:f32,
    champion:f32,
    nestle:f32,
    flour_mills:f32,
    honeywell:f32,
}    

     if k < 10_000_000.0

       impl Percentageleverage {
          fn cadb(&self) -> f32 {
             self.cadbury * 0.05
          } }
       impl Percentageleverage {
          fn champ(&self)->f32 {
             self.champion * 0.05
          } }
       impl Percentageleverage {
          fn nestl(&self)->f32 {
             self.nestle * 0.05
          } }
       impl Percentageleverage {
          fn flour(&self)->f32 {
             self.flour_mills * 0.05
          } }
       impl Percentageleverage {
          fn honey(&self)->f32 {
             self.honeywell * 0.05
          } } 
         
    fn main() {
       let liabiliti = Percentageleverage {
          cadbury: 0.333,
          champion: 0.320,
          nestle: 0.188,
          flour_mills: 0.125,
          honeywell: 0.265,
       };
       println!("The 5% of the percentage leverage of Cadbury Nigeria Plc {}", liabiliti.cadb());
       println!("The 5% of the percentage leverage of Champion Nigeria Plc {}", liabiliti.cadb());
       println!("The 5% of the percentage leverage of Cadbury Nigeria Plc {}", liabiliti.cadb());
       println!("The 5% of the percentage leverage of Cadbury Nigeria Plc {}", liabiliti.cadb());
       println!("The 5% of the percentage leverage of Cadbury Nigeria Plc {}", liabiliti.cadb());
 }
  }

   