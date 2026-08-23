use clap::Parser;

/// A simple example of parsing command line arguments with clap
#[derive(Parser)]
struct Cli {
    /// A required positional argument
    required: u8,

    /// Required argument with only a short form
    #[arg(short)]
    short_only: u8,

    /// Required argument with a short or long form
    #[arg(short, long)]
    can_be_long: u8,

    /// An optional positional argument
    #[arg(default_value = "Hello World")]
    optional_arg: String,
}

fn main() {
    let cli = Cli::parse();

    println!(
        "required: {}, short only: {}, can_be_long: {}, optional_arg: {}",
        cli.required, cli.short_only, cli.can_be_long, cli.optional_arg
    );
}
