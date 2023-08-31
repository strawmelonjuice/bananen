use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use inline_colorization::*;
fn main() {
    /*
    From file:
    // let inhetbestand: &String = &from_file("hi.txt");
    // println!("{}", inhetbestand);


    To file:
    // to_file("testje", "hoi.txt");
    */
    let command = env::args().nth(1).unwrap_or("none".to_string());
    println!("{style_bold}{color_yellow}Bananen!{color_reset}\n{style_reset}By {color_red}Straw{color_green}melon{color_reset}{color_yellow}juice Mar.{color_reset}\n");
    if command == "none" {
        println!("No command specified. Use `bananen help` for help.");
        process::exit(1);
    } else {
        println!("Command used: {}", command);
    }
    let _a = env::args().nth(2).unwrap_or("".to_string());
    let _b = env::args().nth(3).unwrap_or("".to_string());
    let _c = env::args().nth(4).unwrap_or("".to_string());
    let cd = env::current_dir()
    .expect("Expected a valid working directory.");

    let _configfile: String = format!("{0}/bananen.json", cd.display());
    {
        if command == "help" {
            if _a == "" || _a == "1" {
            println!("\n{color_yellow}Bananen{color_reset} help -- page {style_underline}1{style_reset} of 1.
            {style_bold}add{style_reset}     Add new changes to unreleased.
                    Example usage: `{color_yellow}bananen{color_reset} add \"Fixed all the things\"`

            {style_bold}dub{style_reset}     Name unreleased changes into a release.
                    Example usage: `{color_yellow}bananen{color_reset} dub \"V1.29.3\"`

            {style_bold}set{style_reset}     Change config in {_configfile}
                    Example usage: `{color_yellow}bananen{color_reset} dub \"V1.29.3\"`

            {style_bold}help{style_reset}    Display this page.
            ");
            }
        }
    }
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
        .expect("We still need a file!");
    return contents;
}
