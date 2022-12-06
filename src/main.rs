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

    use std::time::Instant;
    let now = Instant::now();
    match &func[..] {
        // "d00s1" => solutions::day00::d00s1(submit),
        // "d00s2" => solutions::day00::d00s2(submit),
        "d1s1" => solutions::day1::d1s1(submit),
        "d1s2" => solutions::day1::d1s2(submit),
        "d2s1" => solutions::day2::d2s1(submit),
        "d2s2" => solutions::day2::d2s2(submit),
        "d3s1" => solutions::day3::d3s1(submit),
        "d3s2" => solutions::day3::d3s2(submit),
        "d4s1" => solutions::day4::d4s1(submit),
        "d4s2" => solutions::day4::d4s2(submit),
        "d5s1" => solutions::day5::d5s1(submit),
        "d5s2" => solutions::day5::d5s2(submit),
        "d6s1" => solutions::day6::d6s1(submit),
        "d6s2" => solutions::day6::d6s2(submit),

        "d2s1rev" => solutions::day2rev::d2s1rev(submit),
        "d2s2rev" => solutions::day2rev::d2s2rev(submit),
        "d3s1rev" => solutions::day3rev::d3s1rev(submit),
        "d3s2rev" => solutions::day3rev::d3s2rev(submit),
        "d5s1rev" => solutions::day5rev::d5s1rev(submit),
        "d5s2rev" => solutions::day5rev::d5s2rev(submit),
        _ => {
            println!("Invalid argument.")
        }
    }
    println!(
        "{}\n",
        format!("Execution time: {:.2?}", now.elapsed())
            .blink()
            .on_white()
    );
}
