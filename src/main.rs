use ron::de::{SpannedError, from_str};
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct ColorSet {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
}

#[derive(Debug, Deserialize)]
struct ColorData {
    name: String,
    foreground: String,
    cursor: String,
    normal: ColorSet,
    bright: ColorSet,
    dim: ColorSet,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename.ron>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Unable to read file");

    let color_data_result: Result<ColorData, SpannedError> = from_str(&contents);

    match color_data_result {
        Ok(color_data) => {
            let kitty_config = format!(
                "# {} made from cosmic-kitty by TallenPeli\n\nbackground\t{}\nforeground\t{}\ncursor\t{}\nselection_background\t{}\nselection_foreground\t{}\n\n# Black to Light White\ncolor0\t{}\ncolor8\t{}\n\n# Primary Colors\ncolor1\t{}\t\ncolor9\t{}\n\ncolor2\t{}\ncolor10\t{}\n\ncolor3\t{}\ncolor11\t{}\n\ncolor4\t{}\ncolor12\t{}\n\ncolor5\t{}\ncolor13\t{}\n\ncolor6\t{}\ncolor14\t{}\n\ncolor7\t{}\ncolor15\t{}\n\n# Dim Colors\ncolor16\t{}\ncolor17\t{}\ncolor18\t{}\ncolor19\t{}\ncolor20\t{}\ncolor21\t{}\ncolor22\t{}\ncolor23\t{}",
                color_data.name,
                color_data.normal.black,
                color_data.foreground,
                color_data.foreground,
                color_data.cursor,
                color_data.normal.black,
                color_data.normal.black,
                color_data.bright.black,
                color_data.normal.red,
                color_data.bright.red,
                color_data.normal.green,
                color_data.bright.green,
                color_data.normal.yellow,
                color_data.bright.yellow,
                color_data.normal.blue,
                color_data.bright.blue,
                color_data.normal.magenta,
                color_data.bright.magenta,
                color_data.normal.cyan,
                color_data.bright.cyan,
                color_data.normal.white,
                color_data.bright.white,
                color_data.dim.black,
                color_data.dim.red,
                color_data.dim.green,
                color_data.dim.yellow,
                color_data.dim.blue,
                color_data.dim.magenta,
                color_data.dim.cyan,
                color_data.dim.white,
            );

            let output_filename = format!("{}-kitty.conf", color_data.name);
            let mut output_file =
                fs::File::create(output_filename).expect("Unable to create output file");
            output_file
                .write_all(kitty_config.as_bytes())
                .expect("Unable to write to output file");

            println!("Kitty config generated successfully.");
        }
        Err(e) => {
            eprintln!("Error parsing RON: {}", e);
            std::process::exit(1);
        }
    }
}
