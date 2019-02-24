fn main() {
    let result = algorithm(200, 480);
    println!("{:?}", result);
}

fn algorithm(mut a: u64, mut b: u64) -> u64{
    while a != 0 && b != 0{
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    let result = a + b;
    result
}
