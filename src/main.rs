mod cli;
mod exercises;

fn main() {
    let exercises = cli::cli_get_exercises();

    for (day, (ex, input)) in exercises {
        println!("Day {} solution: {}", day, ex.solve(&input));
    }
}
