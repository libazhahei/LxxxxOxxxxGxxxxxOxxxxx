mod ast;
mod commands;
mod heap;
mod parsers;
mod program;
#[cfg(test)]
mod tests;
mod tokens;
mod transpiler;
mod turtle;

use clap::Parser;
use program::Program;
use std::fs;
use unsvg::Image;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,
    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    // let image_path = std::path::PathBuf::from("/home/shilong/cs6991/assign/rslogo/output.svg");
    // let height = 200;
    // let width = 200;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        println!("{:?}", err);
        std::process::exit(1);
    });
    let image = Image::new(width, height);
    let mut program = Program::parse_logo(&contents, image);
    program.run();
    println!("{}", program.to_python());
    let image = program.image();

    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}
