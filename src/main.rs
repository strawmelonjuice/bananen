use inline_colorization::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    let command = env::args().nth(1).unwrap_or("none".to_string());
    println!("{style_bold}{color_yellow}Bananen! 🍌{color_reset} v{VERSION}\n{style_reset}By {color_red}Straw{color_green}melon{color_yellow}juice {color_magenta}Mar{color_reset}.");
    if command == "none" {
        println!(
            "{color_red}ERROR:{color_reset} No command specified. Use `{color_yellow}bananen{color_reset} {color_blue}help{color_reset}` for help."
        );
        process::exit(1);
    } else {
        // println!("Command used: {}", command);
    }
    let _bananen_version: &str = env!("CARGO_PKG_VERSION");
    let _a = env::args().nth(2).unwrap_or("".to_string());
    let _b = env::args().nth(3).unwrap_or("".to_string());
    let _c = env::args().nth(4).unwrap_or("".to_string());
    let savefile: &str = &get_save_file_path();
    if command == "help" {
        if _a == "" || _a == "1" {
            println!(
                r#"
    {color_yellow}Bananen{color_reset} help -- page {style_underline}1{style_reset} of 1.
        {style_bold}{color_blue}add{color_reset}{style_reset}     Add new changes to unreleased.
            Usage: `{color_yellow}bananen{color_reset} {color_blue}add{color_reset} <type> <title> [--breaking]`
                Options:
                    types: removal, fix, addition, update
                    flags:
                    --breaking: Will add a breaking warning to the changelog.
                Example usage: `{color_yellow}bananen{color_reset} {color_blue}add{color_reset} --breaking "Fixed all the things"`

        {style_bold}{color_blue}dub{color_reset}{style_reset}     Name unreleased changes into a release.
                Example usage: `{color_yellow}bananen{color_reset} {color_blue}dub{color_reset} "V1.29.3"`

        {style_bold}{color_blue}set{color_reset}{style_reset}     Change config in `{color_cyan}{savefile}{color_reset}`.
                Example usage: `{color_yellow}bananen{color_reset} {color_blue}set{color_reset} changelogfile logchange.MD`

        {style_bold}{color_blue}init{color_reset}{style_reset}    Initialise the current folder with a brand new `{color_cyan}{savefile}{color_reset}`.

        {style_bold}{color_blue}help{color_reset}{style_reset}    Display this page."#
            );
            process::exit(0);
        }
    }
    if command == "init" {
        if Path::new(&get_save_file_path()).exists() {
            if _a != "--proceed" {
                println!(
                    r#"
                {color_red}Warning:{color_reset}
                `{color_cyan}{savefile}{color_reset}` already exists.
                Use with {color_black}{bg_white}--proceed{color_reset}{bg_reset} if you're willing to overwrite it.
                Canceled!"#
                );
                process::exit(0);
            }
        }
        let bananen_version: &str = env!("CARGO_PKG_VERSION");
        let clean_save_data = json::object!{
    "$schema": "https://json.schemastore.org/prettierrc",
    "bananen_version": bananen_version,
  "config": {
    "changelogfile": "./changelog.md"
  },
  "entries": {}
};
        println!("Writing clean setup to `{color_cyan}{savefile}{color_reset}`!");
        to_file(&clean_save_data.dump(), &savefile);
        process::exit(0);
    }
    if !Path::new(&get_save_file_path()).exists() {
        println!(
                "{color_red}ERROR:{color_reset} No `{color_cyan}{savefile}{color_reset}` found. Use `{color_yellow}bananen{color_reset} {color_blue}init{color_reset}` to create one."
            );
        process::exit(1);
    }
    let _savedata: json::JsonValue = load_save_file();
    println!(
                "{color_red}ERROR:{color_reset} Unknown command. Use `{color_yellow}bananen{color_reset} {color_blue}help{color_reset}` for help."
            );
    process::exit(1);
}

fn to_file(contents: &str, file: &str) {
    to_file2(contents, file)
        .map_err(|err| println!("{:?}", err))
        .ok();
}
fn to_file2(contents: &str, file: &str) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn from_file(file: &str) -> String {
    let mut o = File::open(file).expect("We need a file!");
    let mut contents = String::new();
    o.read_to_string(&mut contents)
        .expect("ERROR: Expected I could read that file!");
    return contents;
}
fn load_save_file() -> json::JsonValue {
    let savefile = &get_save_file_path();
    let unparsed_confi = from_file(savefile);
    let unparsed_config: &str = unparsed_confi.as_str();
    let parsedsavefile = json::parse(unparsed_config)
        .expect("ERROR: Expected I could understand {savefile}! If you don't mind resetting everything bananen has done, please reinitialise it with `bananen init`.");
    return parsedsavefile;
}
fn get_save_file_path() -> std::string::String {
    let cd = env::current_dir().expect("ERROR: Expected a valid working directory.");

    let savefile: String = format!("{0}/bananen.json", cd.display());
    return savefile;
}
