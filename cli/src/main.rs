use clap::{Parser, Subcommand, command};

#[derive(Debug, Parser)]
#[command(name = "imaged")]
#[command(about = "Multithreaded image encryption and decryption tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Encrypt {
        // method: CipherMethod,
        image_path: String,
        key: String,
    },
    Decrypt {
        // method: CipherMethod,
        image_path: String,
        key: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Encrypt { image_path, key } => {}
        Commands::Decrypt { image_path, key } => {}
    }
}
