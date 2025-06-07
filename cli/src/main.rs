use core::{ArnoldCat, HyperChaosSVD, ImageCipher};
use std::path::Path;

use clap::{Parser, Subcommand, ValueEnum, command};
use image::codecs::png::PngEncoder;

#[derive(Debug, Parser)]
#[command(name = "imaged")]
#[command(about = "Multithreaded image encryption and decryption tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
enum CipherMethod {
    ArnoldCat,
    HenonMap,
    HyperChaosSVD,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Encrypt {
        method: CipherMethod,
        image_path: String,
        output_path: String,
        key: String,
    },
    Decrypt {
        method: CipherMethod,
        image_path: String,
        output_path: String,
        key: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Encrypt {
            method,
            image_path,
            output_path,
            key,
        } => {
            let image = image::open(Path::new(&image_path)).unwrap();

            let enc_image = match method {
                CipherMethod::ArnoldCat => ArnoldCat::encrypt(image, key),
                CipherMethod::HenonMap => image,
                CipherMethod::HyperChaosSVD => HyperChaosSVD::encrypt(image, key),
            };
            enc_image.save(Path::new(&output_path)).unwrap();
        }
        Commands::Decrypt {
            method,
            image_path,
            output_path,
            key,
        } => {}
    }
}
