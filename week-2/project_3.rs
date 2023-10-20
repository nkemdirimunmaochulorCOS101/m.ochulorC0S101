fn main() {
   let p:f64 = 210_000.0;
   let r:f64 = 5.0;
   let n:f64 = 3.0;

   //depreciation
   let v = 1.0 - (r/100.0);
   let q = f64::powf(v, n);
   let a = p * q;
   println!("Amount is {}", a);
   let dep = a - p;
   println!("depreciation is {}", dep);

}

