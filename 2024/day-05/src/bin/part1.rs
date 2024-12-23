use day_05::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = std::fs::read_to_string("day-05/input.txt")
        .map_err(|e| miette::miette!("Read input file: {e}"))?;
    let result = process(file.as_str()).context("process part 1")?;

    println!("{}", result);
    Ok(())
}
