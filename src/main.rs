fn main() {
    for i in 1..=100 {
        let mut output = String::new();
        if i % 3 == 0 {
            output += "Fiz";
        }
        if i % 5 == 0 {
            output += "Buzz";
        }
        if output != "" {
            println!("{}", output);
        } else {
            println!("{}", i);
        }
     }
}
