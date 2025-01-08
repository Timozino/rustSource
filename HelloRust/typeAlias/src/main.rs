type Meter = i32;

fn main(){
    let distance: Meter = 1000;
    println!("Distance: {} meters", distance);

    let speed: f64 = distance as f64 / 10.0;
    println!("Speed: {} meters per second", speed);
}