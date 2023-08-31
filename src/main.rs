use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use inline_colorization::*;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // To file:
    // to_file("testje", "hi.txt");


    // From file:
    // let _inhetbestand: &String = &from_file("hi.txt");
    // println!("{}", inhetbestand);

    let command = env::args().nth(1).unwrap_or("none".to_string());
    println!("{style_bold}{color_yellow}Bananen! 🍌{color_reset} v{VERSION}\n{style_reset}By {color_red}Straw{color_green}melon{color_yellow}juice {color_magenta}Mar.{color_reset}");
    if command == "none" {
        println!("{color_red}ERROR:{color_reset} No command specified. Use `bananen help` for help.");
        process::exit(1);
    } else {
        // println!("Command used: {}", command);
    }
    let _a = env::args().nth(2).unwrap_or("".to_string());
    let _b = env::args().nth(3).unwrap_or("".to_string());
    let _c = env::args().nth(4).unwrap_or("".to_string());
    let cd = env::current_dir()
    .expect("{color_red}ERROR:{color_reset} Expected a valid working directory.");

    let _configfile: String = format!("{0}/bananen.json", cd.display());
    {
        if command == "help" {
            if _a == "" || _a == "1" {
            println!(r#"
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

            {style_bold}{color_blue}set{color_reset}{style_reset}     Change config in {_configfile}
                    Example usage: `{color_yellow}bananen{color_reset} {color_blue}set{color_reset} changelogfile logchange.MD`

            {style_bold}{color_blue}help{color_reset}{style_reset}    Display this page."#);
            }
        }
    }
}
#[allow(dead_code)]
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
#[allow(dead_code)]
fn from_file(file: &str) -> String {
    let mut o = File::open(file).expect("We need a file!");
    let mut contents = String::new();
    o.read_to_string(&mut contents)
        .expect("{color_red}ERROR:{color_reset} Expected I could read that file!");
    return contents;
}
