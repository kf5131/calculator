fn main() {
    println!("Welcome to Basic Calculator!");
    println!("Enter an expression in the format: number operator number");
    println!("Supported operators: + - * /");
    println!("Example: 5 + 3");

    loop {
        let mut input = String::new();
        
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 3 {
            println!("Invalid input! Please use format: number operator number");
            continue;
        }

        let num1: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("First input must be a number!");
                continue;
            }
        };

        let operator = parts[1];

        let num2: f64 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Second input must be a number!");
                continue;
            }
        };

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Cannot divide by zero!");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Invalid operator! Use +, -, *, or /");
                continue;
            }
        };

        println!("Result: {} {} {} = {}", num1, operator, num2, result);
    }
}
