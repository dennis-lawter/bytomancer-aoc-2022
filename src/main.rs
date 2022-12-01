use colored::Colorize;
use std::env;

mod input;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let func = args.get(1).expect("Must provide a runtime argument.");
    let mut submit = false;
    match args.get(2) {
        Some(arg) => {
            submit = arg == "--submit" || arg == "-s";
        }
        None => {}
    }
    println!(
        "\n{}\n",
        format!(
            "    Solving {}",
            format!(" {} ", func).black().on_yellow().bold()
        )
        .bold()
        .on_blue()
    );

    match &func[..] {
        "d1s1" => solutions::day1::d1s1(submit),
        "d1s2" => solutions::day1::d1s2(submit),
        _ => {
            println!("Invalid argument.")
        }
    }
}
