use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Command-line interface for ONNX"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    RightTriangle {
        a: u64,
        b: u64,
        c: u64,
    },
}

//invoke lib.rs using onnx_demo namespace
//use onnx_demo::run;
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::RightTriangle {a, b, c}) => {
            if (a*a + b*b) == c*c {
                let result = "Yes";
                println!("Is this triangle a right triangle? {}", result);
            }else{
                let result = "No";
                println!("Is this triangle a right triangle? {}", result);
            }
        }
        None => println!("No subcommand was used"),
    }
}