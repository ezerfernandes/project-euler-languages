fn main() {
    let mut triangular_number: u32 = 1;
    let mut n: u32 = 1;

    while divisor_count(triangular_number) <= 500 {
        n += 1;
        triangular_number += n;
    }
    println!("{}", triangular_number);
}


fn divisor_count(number: u32) -> u32 {
    let mut total;
    let mut count = 0;
    let mut num = number;

    while num % 2 == 0 {
        count += 1;
        num /= 2;
    }
    total = count + 1;

    let mut factor = 3;
    while num > 1 {
        count = 0;
        while num % factor == 0 {
            count += 1;
            num /= factor;
        }
        total *= count + 1;
        factor += 2;
    }

    total
}