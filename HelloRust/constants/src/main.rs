const TAX_RATE : f64= 0.07;

fn main(){
    let income: f64= 100000.00;
    let total : f64;

    total=income - (income*TAX_RATE);
    println!("my tax rate is {TAX_RATE}");
    println!("MY INCOME IS {total}");
}