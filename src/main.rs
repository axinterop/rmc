use argh::FromArgs;
use rmc::Markdown;

#[derive(FromArgs)]
/// rmc - Rust Markdown Compiler
struct Args {
    /// input file with markdown content
    #[argh(positional)]
    input: String,

    /// output file where HTML will be saved
    #[argh(positional)]
    output: String,
}

fn main() -> std::io::Result<()> {
    let args: Args = argh::from_env();
    let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
    let html = Markdown::parse(&input);
    std::fs::write(&args.output, html).expect("Failed to write to output file");
    println!("Successfully converted {} to {}", args.input, args.output);
    Ok(())
}
