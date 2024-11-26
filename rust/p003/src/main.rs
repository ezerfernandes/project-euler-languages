fn main() {
    let mut n: u64 = 600851475143;
    let mut largest_factor: u64 = 0;

    while n % 2 == 0 {
        largest_factor = 2;
        n /= 2;
    }

    let mut i = 3;
    while i <= (n as f64).sqrt() as u64 {
        while n % i == 0 {
            largest_factor = i;
            n /= i;
        }
        i += 2;
    }

    if n > largest_factor {
        largest_factor = n;
    }

    println!("{}", largest_factor);
}
