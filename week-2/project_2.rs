fn main() {
   let toshiba = 450_000.0;
   let mac = 1_500_00.0;
   let hp = 750_000.0;
   let dell = 2_850_000.0;
   let acer = 250_000.0;

   // average
   let s = (toshiba * 2.0 ) + ( mac * 1.0 ) + ( hp * 3.0 ) + ( dell * 3.0 ) + ( acer * 1.0 );
   println!(" Sum is {}", s);
   let a = s / 10.0;
   println!(" average is {}", a);
}