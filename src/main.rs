mod commands;
mod utils;

use clap::{ArgAction, Parser, Subcommand};
use commands::*;
use utils::*;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    propagate_version = true,
    arg_required_else_help(true)
)]
pub struct Cli {
    #[command(subcommand)]
    subcmd: SubCommands,
    /// Scan root path
    #[arg(short, long, action = ArgAction::Append)]
    root_path: String,
    /// Scan pattern
    #[arg(long, action = ArgAction::Append)]
    pattern: String,
}

#[derive(Subcommand)]
enum SubCommands {
    Image,
    Imagesize,
    Imageinfo,
    Image2,
    Png,
    Lodepng,
    Jpegdecoder,
    Psd,
    Tinytga,
    Tiff,
}

fn main() {
    let args = Cli::parse();
    let files = scan_files(&args.root_path, &args.pattern);

    match args.subcmd {
        SubCommands::Image => image::head_info(files),
        SubCommands::Imagesize => imagesize::head_info(files),
        SubCommands::Imageinfo => imageinfo::head_info(files),
        SubCommands::Image2 => image2::head_info(files),
        SubCommands::Png => png::head_info(files),
        SubCommands::Lodepng => lodepng::head_info(files),
        SubCommands::Jpegdecoder => jpeg_decoder::head_info(files),
        SubCommands::Psd => psd::head_info(files),
        SubCommands::Tinytga => tinytga::head_info(files),
        SubCommands::Tiff => tiff::head_info(files),
    };
}
