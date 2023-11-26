fn main() {
    println!("pi value is {}",get_pt());
}

fn get_pt()->f64 {
    let a:f64 = 22.0;
    let b:f64 = 7.0;
    let c:f64 = a / b;
    return c;
}
