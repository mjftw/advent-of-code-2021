mod cli;
mod day1;

fn main() {
    let day_inputs = cli::cli_get_day_inputs();

    if let Some(raw_input) = day_inputs.get(&1) {
        let input = day1::parse_input(raw_input);
        println!("Day 1 answer: {}", day1::solve(&input));
    }
}
