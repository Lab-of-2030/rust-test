mod question1;
mod question2;
mod question3;
mod question4;
mod question5;
mod question6;

use std::env;

use question2::Student;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <questionX>", args[0]);
        return;
    }

    match args[1].as_str() {
        "question1" => {
            question1::run();
        }
        "question2" => {
            let student = Student::new("Alice", 18, 95.5);
            student.run();
        }
        "question3" => {
            question3::run();
        }
        "question4" => {
            if args.len() < 3 {
                eprintln!("Usage: {} <input_file>", args[0]);
                std::process::exit(1);
            }
        
            let input_path = &args[2];
            let output_path = "output.txt";
        
            if let Err(err) = question4::run(input_path, output_path) {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
        "question5" => {
            question5::run();
        }
        "question6" => {
            question6::run(&args);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}