use clap::Parser;

/// Search for a pattern in a file and display the lines that contains it.
#[derive(Parser)]
struct Cli {
    /// the pattern to look for
    pattern: String,
    
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // println!("args.path.display(): {}", args.path.display());
    // println!("args.pattern: {}", args.pattern);

    let content = std::fs::read_to_string(args.path).expect("could not read file");

    // println!("content");
    // println!("---");
    // println!("{}", content);
    // println!("---");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
