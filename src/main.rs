use inline_colorization::*;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

#[derive(Deserialize)]
struct BananenSaveData {
    main: BananenSaveDataMain,
    config: BananenConfig,
    changes_md: BananensavedChanges,
}

#[derive(Deserialize)]
struct BananenSaveDataMain {
    bananen_version: String,
}
#[derive(Deserialize)]
struct BananenConfig {
    changelogfile: String,
}
#[derive(Deserialize)]
struct BananensavedChanges {
    unreleased: String,
    released: String,
}
const VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    let me_bin = env::args().nth(0).unwrap_or("unknown".to_string());
    // println!("{}", me_bin);
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
    let _a: String = env::args().nth(2).unwrap_or("".to_string());
    let _b: String = env::args().nth(3).unwrap_or("".to_string());
    let _c: String = env::args().nth(4).unwrap_or("".to_string());
    let _d: String = env::args().nth(5).unwrap_or("".to_string());
    let savefile: &str = &get_save_file_path();
    if command == "help" || command == "h" || command == "--help" || command == "-h" {
        if _a == "" || _a == "1" {
            println!(
                r#"
    {color_yellow}Bananen{color_reset} help -- page {style_underline}1{style_reset} of 1.
        {style_bold}{color_blue}add{color_reset}{style_reset}     Add new changes to unreleased.
            Usage: `{color_yellow}{me_bin}{color_reset} {color_blue}add{color_reset} <type> <title> {color_black}{bg_white}[--breaking]{color_reset}{bg_reset}`
                Options:
                    types: removal, fix, addition, update
                    flags:
                    --breaking: Will add a breaking warning to the changelog.
                Example usage: `{color_yellow}{me_bin}{color_reset} {color_blue}add{color_reset} {color_black}{bg_white}--breaking{color_reset}{bg_reset} "Fixed all the things"`

        {style_bold}{color_blue}dub{color_reset}{style_reset}     Name unreleased changes into a release.
                Example usage: `{color_yellow}{me_bin}{color_reset} {color_blue}dub{color_reset} "V1.29.3"`

        {style_bold}{color_blue}regen{color_reset}{style_reset}     Regenerate changelog from index in '{color_cyan}{savefile}{color_reset}'.

        {style_bold}{color_blue}init{color_reset}{style_reset}    Initialise the current folder with a brand new '{color_cyan}{savefile}{color_reset}'.

        {style_bold}{color_blue}help{color_reset}{style_reset}    Display this page."#
            );
            process::exit(0);
        }
    }
    if command == "init" || command == "i" {
        if Path::new(&get_save_file_path()).exists() {
            if _a != "--proceed" {
                println!(
                    r#"
    {color_red}Warning:{color_reset}
    '{color_cyan}{savefile}{color_reset}' already exists.
    Use with {color_black}{bg_white}--proceed{color_reset}{bg_reset} if you're willing to overwrite it."#
                );
                process::exit(0);
            }
        }
        let clean_save_data = format!(
            r#"[main]
bananen_version = "{_bananen_version}"

[config]
changelogfile = "changelog.md"

[changes_md]
unreleased= """"""
released= """""""#
        );
        println!("Writing new save data to '{color_cyan}{savefile}{color_reset}'!");
        to_file(&clean_save_data, &savefile);
        process::exit(0);
    }
    if !Path::new(&get_save_file_path()).exists() {
        println!(
                "{color_red}ERROR:{color_reset} No '{color_cyan}{savefile}{color_reset}' found. Use `{color_yellow}{me_bin}{color_reset} {color_blue}init{color_reset}` to create one."
            );
        process::exit(1);
    }
    let _savedata = load_save_file();
    if command == "add" || command == "a" {
        if _a == "" || _b == "" {
            println!(
                "{color_red}ERROR:{color_reset} No argument found for this {color_blue}add{color_reset}ition what do I need to add?"
            );
            process::exit(1);
        }
        let additiontype: String = _a.clone();
        if additiontype == "add" || additiontype == "a" {
            println!("(Accepted short `{additiontype}` as `addition`)");
        }
        let additiontype = if additiontype == "add" || additiontype == "a" {
            "addition"
        } else {
            &additiontype
        };
        if additiontype == "up" || additiontype == "u" {
            println!("(Accepted short `{additiontype}` as `update`)");
        }
        let additiontype = if additiontype == "up" || additiontype == "u" {
            "update"
        } else {
            &additiontype
        };
        if !(additiontype == "removal"
            || additiontype == "fix"
            || additiontype == "addition"
            || additiontype == "update")
        {
            println!(
                "{color_red}ERROR:{color_reset} Incorrect type: `{additiontype}`! What kinda {color_blue}add{color_reset}ition is this?"
            );
            process::exit(1);
        }
        let savedata = load_save_file();
        let changelogfile = format!("{}", savedata.config.changelogfile);

        println!(
            "{color_green}{0}{color_reset}: '{1}' --> unreleased@\"{2}\"",
            additiontype,
            _b,
            return_pathslashfile(&changelogfile)
        );
        let breakingwarn = if _c == "--breaking" {
            "<span style=\"font-color: red\">BREAKING!</span>"
        } else {
            ""
        };
        let oldunreleasedchanges: String = savedata.changes_md.unreleased;
        let newunreleasedchanges: String = format!(
            "- {breakingwarn} **{additiontype}**: {_b}\n{oldunreleasedchanges}"
        );
        let releasedchanges: String = savedata.changes_md.released;
        let new_md: String = format!(
            "# Changelog\n\n## Unreleased\n\n{newunreleasedchanges}\n\n\n{releasedchanges}\n\n\n<hr>This file was auto generated by [<span style=\"background-color: #24273a; color: yellow\">Bananen! 🍌</span>](https://github.com/strawmelonjuice/bananen/) v{0}",
            _savedata.main.bananen_version
        );
        let new_savedata: String = format!(
            r#"[main]
bananen_version = "{_bananen_version}"

[config]
changelogfile = "{changelogfile}"

[changes_md]
unreleased= """{newunreleasedchanges}"""
released= """{releasedchanges}""""#
        );
        to_file(&new_md, return_pathslashfile(&changelogfile).as_str());
        to_file(&new_savedata, &savefile);
        process::exit(0);
    }
if command == "regen" || command == "r" {
    let changelogfile = format!("{}", _savedata.config.changelogfile);
    let md: String = format!(
            "# Changelog\n\n## Unreleased\n\n{0}\n\n\n{1}\n\n\n<hr>This file was auto generated by [<span style=\"background-color: #24273a; color: yellow\">Bananen! 🍌</span>](https://github.com/strawmelonjuice/bananen/) v{2}",
            _savedata.changes_md.unreleased,
            _savedata.changes_md.released,
            _savedata.main.bananen_version
        );
    to_file(&md, return_pathslashfile(&changelogfile).as_str());
    println!("{color_green}Regenerated {color_reset}'{0}'!",return_pathslashfile(&changelogfile));
    process::exit(0);
}
    println!(
                "{color_red}ERROR:{color_reset} Unknown command. Use `{color_yellow}{me_bin}{color_reset} {color_blue}help{color_reset}` for help."
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
    let expectationerror = format!("{color_red}ERROR:{color_reset} Looks like '{file}' isn't what I expected. I expected a file there.");
    let mut o = File::open(file).expect(&expectationerror);
    let mut contents = String::new();
    let expectationerror = format!("{color_red}ERROR:{color_reset} Looks like '{file}' isn't what I expected. I could not read that file.");
    o.read_to_string(&mut contents).expect(&expectationerror);
    return contents;
}
fn load_save_file() -> BananenSaveData {
    let savefile = &get_save_file_path();
    let unparsed_confi = from_file(savefile);
    let unparsed_config: &str = unparsed_confi.as_str();
    let me_bin = env::args().nth(0).unwrap_or("unknown".to_string());
    let expectationerror = format!("{color_red}ERROR:{color_reset} Expected I could understand '{color_cyan}{savefile}{color_reset}'!\nIf you don't mind resetting everything bananen has done, please reinitialise it with:\n`{color_yellow}{me_bin}{color_reset} {color_blue}init{color_reset} {color_black}{bg_white}--proceed{color_reset}{bg_reset}`.");
    let parsedsavefile: BananenSaveData = toml::from_str(unparsed_config).expect(&expectationerror);
    return parsedsavefile;
}
fn get_save_file_path() -> String {
    return return_pathslashfile("bananen.toml");
}
fn return_pathslashfile(file: &str) -> String {
    let expectationerror =
        format!("{color_red}ERROR:{color_reset} Expected a valid working directory.");
    let cd = env::current_dir().expect(&expectationerror);
    let returns: String = format!("{0}/{1}", cd.display(), file);
    return returns;
}
