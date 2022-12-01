use colored::Colorize;

pub mod day1;

pub fn final_answer<T: std::fmt::Display>(answer: T) {
    println!(
        "\n{}\n",
        format!(
            "    Solution {}",
            format!(" {} ", answer).black().on_yellow().bold()
        )
        .bold()
        .on_blue()
    );
}
