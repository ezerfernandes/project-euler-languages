fn main() {
    // difference is the difference between the square of the sum and the sum of the squares.
    let mut difference: u32 = 0;
    let mut rest: u32 = (1..101).sum();
    for n in 1..101 {
        rest -= n;
        difference += n * rest;
    }
    difference = difference * 2;
    println!("{}", difference);

}
