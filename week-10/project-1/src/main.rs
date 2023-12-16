struct Electroniclaptops {
    hp_laptops:u32,
    ibm_laptops:u32,
    toshiba_laptops:u32,
    dell_laptops:u32
}
impl Electroniclaptops {
    fn total_cost(&self)->u32 {

        (self.hp_laptops * 3) + (self.ibm_laptops * 3) + (self.toshiba_laptops * 3) + (self.dell_laptops * 3)
    }
}
fn main() {
    let cost = Electroniclaptops {
        hp_laptops:650_000,
        ibm_laptops:755_000,
        toshiba_laptops:550_000,
        dell_laptops:850_000
    };

    println!("the total cost for all the alptops from each brand is {}", cost.total_cost());
}
