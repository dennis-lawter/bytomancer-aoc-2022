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
        // INITIAL SOLUTIONS

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
        "d7s1" => solutions::day7::d7s1(submit),
        "d7s2" => solutions::day7::d7s2(submit),
        "d8s1" => solutions::day8::d8s1(submit),
        "d8s2" => solutions::day8::d8s2(submit),
        "d9s1" => solutions::day9::d9s1(submit),
        "d9s2" => solutions::day9::d9s2(submit),
        "d10s1" => solutions::day10::d10s1(submit),
        "d10s2" => solutions::day10::d10s2(submit),
        "d11s1" => solutions::day11::d11s1(submit),
        "d11s2" => solutions::day11::d11s2(submit),
        "d12s1" => solutions::day12::d12s1(submit),
        "d12s2" => solutions::day12::d12s2(submit),
        "d13s1" => solutions::day13::d13s1(submit),
        "d13s2" => solutions::day13::d13s2(submit),
        "d14s1" => solutions::day14::d14s1(submit),
        "d14s2" => solutions::day14::d14s2(submit),

        // REVISED APPROACHES

        // "d00s1rev" => solutions::day00rev::d00s1rev(submit),
        // "d00s2rev" => solutions::day00rev::d00s2rev(submit),
        "d2s1rev" => solutions::day2rev::d2s1rev(submit),
        "d2s2rev" => solutions::day2rev::d2s2rev(submit),
        "d3s1rev" => solutions::day3rev::d3s1rev(submit),
        "d3s2rev" => solutions::day3rev::d3s2rev(submit),
        "d5s1rev" => solutions::day5rev::d5s1rev(submit),
        "d5s2rev" => solutions::day5rev::d5s2rev(submit),
        "d6s1rev" => solutions::day6rev::d6s1rev(submit),
        "d6s2rev" => solutions::day6rev::d6s2rev(submit),

        // VISUALIZATIONS

        // "d00s1vis" => solutions::day00vis::d00s1vis(submit),
        // "d00s2vis" => solutions::day00vis::d00s2vis(submit),
        "d7s1vis" => solutions::day7vis::d7s1vis(submit),
        "d7s2vis" => solutions::day7vis::d7s2vis(submit),
        "d8s1vis" => solutions::day8vis::d8s1vis(submit),
        "d8s2vis" => solutions::day8vis::d8s2vis(submit),
        "d9s1vis" => solutions::day9vis::d9s1vis(submit),
        "d9s2vis" => solutions::day9vis::d9s2vis(submit),
        "d12s1vis" => solutions::day12vis::d12s1vis(submit),
        "d12s2vis" => solutions::day12vis::d12s2vis(submit),

        // ERR
        invalid => {
            println!(
                "{}\n",
                format!("Invalid argument: {}", invalid.bold()).on_red()
            )
        }
    }
    println!(
        "{}\n",
        format!("Execution time: {:.2?}", now.elapsed())
            .black()
            .on_white()
    );
}
