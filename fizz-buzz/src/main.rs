fn main() {
    for number in 1..101 {
        print!("{number}\t");

        if number % 3 == 0 {
            print!("Fizz");
        }

        if number % 5 == 0 {
            print!("Buzz");
        }

        print!("\n");
    }
}
