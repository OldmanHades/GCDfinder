fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let num1 = 1200;
    let num2 = 764;

    let gcd = gcd(num1, num2);

    println!("The GCD of {} and {} is {}", num1, num2, gcd);
}
