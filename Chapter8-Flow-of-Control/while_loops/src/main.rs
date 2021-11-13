fn main() {
    let mut n = 1;
    
    while n <= 100 {
        let mut print_str = String::new();
        if (n % 3 == 0) {
            print_str.push_str("Fizz");
        }
        if (n % 5 == 0) {
            print_str.push_str("Buzz");
        }
        if print_str.len() == 0 {
            print_str.push_str(&n.to_string());
        }

        println!("{}", print_str);
        n += 1;
    }
}
