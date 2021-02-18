fn input() -> i32 {
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(num) => {
            println!("Число: {}", num);
            return num;
        }
        Err(..) => {
            println!("Это не число было записанно число: 0");
            return 0;
        }
    };
}

struct EuclidsAlgorithmOutput {
    remainders: Vec<i32>,
    integers: Vec<i32>,
    x: Vec<i32>,
    y: Vec<i32>
}

fn solve_euclids_algorithm(number1: i32, number2: i32) -> EuclidsAlgorithmOutput {
    let mut num1: i32 = number1.abs();
    let mut num2: i32 = number2.abs();
    let mut output: EuclidsAlgorithmOutput = EuclidsAlgorithmOutput {
        remainders: vec![num1, num2],
        integers: vec![],
        x: vec![(if number1 > 0 { 1 } else { -1 }), 0],
        y: vec![0, (if number2 > 0 { 1 } else { -1 })],
    };

    if num1 < num2 {
        output.remainders.push(num1);
        output.integers.push(0);
        output.x.push(output.x[output.x.len() - 2]);
        output.y.push(output.y[output.y.len() - 2]);
        num1 = num1 + num2;
        num2 = num1 - num2;
        num1 = num1 - num2;
    }

    while num2 != 0 {
        let remainder: i32 = num1 % num2;
        let integer: i32 = (num1 - remainder) / num2;
        output.remainders.push(remainder);
        output.integers.push(integer);
        output.x.push(output.x[output.x.len() - 1] - integer * output.x[output.x.len() - 2]);
        output.y.push(output.y[output.y.len() - 1] - integer * output.y[output.y.len() - 2]);
        num1 = output.remainders[output.remainders.len() - 2];
        num2 = output.remainders[output.remainders.len() - 1];
    }

    return output;
}

fn print_euclids_algorithm_table(solved_table: EuclidsAlgorithmOutput) {
    print!("\n\t\t    A\t    B{}  НОД", "\t".repeat(solved_table.remainders.len() - 3));
    print!("\nОстатки:\t");
    for num in solved_table.remainders {
        print!("|  {}\t", num);
    }
    print!("\nЦелые делители:\t|\t|\t");
    for num in solved_table.integers {
        print!("|  {}\t", num);
    }
    print!("\nX:\t\t");
    for num in solved_table.x {
        print!("|  {}\t", num);
    }
    print!("\nY:\t\t");
    for num in solved_table.y {
        print!("|  {}\t", num);
    }
    println!();
}

fn main() {
    println!("Введите первое число: ");
    let number1: i32 = input();
    println!("Введите второе число: ");
    let number2: i32 = input();
    print!("Результат: ");
    print_euclids_algorithm_table(solve_euclids_algorithm(number1, number2));
}
