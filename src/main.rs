use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(
    name = "My RPN program",
    version = "1.0.1",
    author = "isdh",
    about = "Super awesome sample RPN calculator",
    help_template = "\
////////////////////////////////////
{name} {version} created by {author}.
this is {about}.
///////////////////////////////////"
)]

struct Opts {
    /// sets the level of verbosity
    #[clap(short,long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>
}

fn main() {
    let opts = Opts::parse();

    // 以下はget_one/get_manyに変わった
    // match matches.value_of("formula_file") {
    match opts.formula_file {
        Some(file) => println!("File speecified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {} ", opts.verbose );
}