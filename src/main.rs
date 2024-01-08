use inline_colorization::{
    bg_reset, bg_white, color_black, color_blue, color_cyan, color_green, color_magenta, color_red,
    color_reset, color_yellow, style_bold, style_reset, style_underline,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

const BANANEN_CONFIG_VERSION: i32 = 3;
// JSON scheme is not used by Bananen, but might be used by editors when configuring.
const JSON_SCHEMA_URL: &str =
    "https://cdn.jsdelivr.net/gh/strawmelonjuice/bananen@latest/json-schemas/v3.json";

#[derive(Deserialize, Debug, Serialize)]
struct BasicBananenSaveData {
    main: BasicBananenSaveDataMain,
}
#[derive(Deserialize, Debug, Serialize)]
struct BasicBananenSaveDataMain {
    bananendata_version: i32,
}
#[derive(Deserialize, Debug, Serialize)]
struct BananenSaveDatav3 {
    main: BananenSaveDataMain,
    config: BananenConfig,
    saved_changes: BananensavedChanges,
}

#[derive(Deserialize, Debug, Serialize)]
struct BananenSaveDataMain {
    bananen_version: String,
    bananendata_version: i32,
}
#[derive(Deserialize, Debug, Serialize)]
struct BananenConfig {
    changelogfile: String,
    rollingrelease: bool,
    customisation: BananenCustomisations,
}
#[derive(Deserialize, Debug, Serialize)]
struct BananensavedChanges {
    unreleased: Vec<Change>,
    released: Vec<ReleasedChanges>,
}
#[derive(Deserialize, Debug, Serialize, Clone)]
struct ReleasedChanges {
    name: String,
    changes: Vec<Change>,
}
#[derive(Deserialize, Debug, Serialize)]
struct BananenCustomisations {
    log_name: String,
    released_name: String,
    unreleased_name: String,
    changetypes: Changetypes,
}
#[derive(Deserialize, Debug, Serialize)]
struct Changetypes {
    addition: Changetype,
    removal: Changetype,
    update: Changetype,
    fix: Changetype,
}
#[derive(Deserialize, Debug, Serialize)]
struct Changetype {
    translation: String,
    color: String,
}
#[derive(Deserialize, Debug, Serialize, Clone)]
struct Change {
    contents: String,
    r#type: i32,
    breaking: bool,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn arpl(n: u8) -> usize {
    if n < 1 {
        n.into()
    } else if env::args()
        .nth(1)
        .unwrap_or(String::from("unknown"))
        .starts_with('-')
        && (env::args().nth(1).unwrap_or(String::from("unknown")) != "--help")
        && (env::args().nth(1).unwrap_or(String::from("unknown")) != "-h")
    {
        (n + 1).into()
    } else {
        n.into()
    }
}
fn printer(c: f32, f: String) {
    if env::args()
        .nth(1)
        .unwrap_or(String::from("unknown"))
        .starts_with("--min")
        || env::args()
            .nth(1)
            .unwrap_or(String::from("unknown"))
            .starts_with("-m")
    {
        if c != 0.0 {
            print!("{}", c);
        }
    } else if env::args().nth(1).unwrap_or(String::from("unknown")) == "--dump-codes" {
        println!("c: {0}    f: {1}", c, f);
    } else {
        println!("{}", f);
    }
}
fn main() {
    let me_bin = env::args().next().unwrap_or(String::from("unknown"));
    let _kivimode: bool = env::args()
        .nth(1)
        .unwrap_or(String::from("unknown"))
        .starts_with("--min");
    let command = env::args().nth(arpl(1)).unwrap_or(String::from("none"));
    printer( 0.0, format!("{style_bold}{color_yellow}Bananen! 🍌{color_reset} v{VERSION}\n{style_reset}By {color_red}Straw{color_green}melon{color_yellow}juice {color_magenta}Mar{color_reset}."));
    if command == "none" {
        printer( 1.1, format!(
            "{color_red}ERROR:{color_reset} No command specified. Use `{color_yellow}bananen{color_reset} {color_blue}help{color_reset}` for help."
        ));
        process::exit(1);
    }
    let _bananen_version: &str = env!("CARGO_PKG_VERSION");
    let _a: String = env::args().nth(arpl(2)).unwrap_or(String::from(""));
    let _b: String = env::args().nth(arpl(3)).unwrap_or(String::from(""));
    let _c: String = env::args().nth(arpl(4)).unwrap_or(String::from(""));
    let _d: String = env::args().nth(arpl(5)).unwrap_or(String::from(""));
    let savefile: &str = &get_save_file_path();

    if command == "help" || command == "h" || command == "--help" || command == "-h" {
        if _a.is_empty() || _a == "1" {
            printer(
                0.0,
                format!(
                    r#"
    {color_yellow}Bananen{color_reset} help -- page {style_underline}1{style_reset} of 1.
        {style_bold}{color_blue}add{color_reset}{style_reset}     Add new changes to unreleased.
            Usage: `{color_yellow}{me_bin}{color_reset} {color_blue}add{color_reset} <type> <title> {color_black}{bg_white}[--breaking]{color_reset}{bg_reset}`
                Options:
                    types: (r)emoval, (f)ix, (a)ddition, (u)pdate
                    flags:
                    --breaking: Will add a breaking warning to the changelog.
                Example usage: `{color_yellow}{me_bin}{color_reset} {color_blue}add{color_reset} "Fixed all the things" {color_black}{bg_white}--breaking{color_reset}{bg_reset}`

        {style_bold}{color_blue}dub{color_reset}{style_reset}     Name unreleased changes into a release.
                Example usage: `{color_yellow}{me_bin}{color_reset} {color_blue}dub{color_reset} "V1.29.3"`

        {style_bold}{color_blue}regen{color_reset}{style_reset}     Regenerate changelog from index in '{color_cyan}{savefile}{color_reset}'.
        

        {style_bold}{color_blue}init{color_reset}{style_reset}    Initialise the current folder with a brand new '{color_cyan}{savefile}{color_reset}'.

        {style_bold}{color_blue}help{color_reset}{style_reset}    Display this page. Use with version to display version.
        
        Note: Using the --minimal tag (in first place ONLY), you'll only get floating point numbers as output, however Bananen will function as usual."#
                ),
            );
            process::exit(0);
        };

        if _a == "v" || _a == "version" || _a == "ver" {
            println!("{}", _bananen_version);
            process::exit(0)
        }
    }
    if command == "init" || command == "i" {
        if Path::new(&get_save_file_path()).exists() && _a != "--proceed" {
            printer(
                2.1,
                format!(
                    r#"
    {color_red}Warning:{color_reset}
    '{color_cyan}{savefile}{color_reset}' already exists.
    Use with {color_black}{bg_white}--proceed{color_reset}{bg_reset} if you're willing to overwrite it."#
                ),
            );
            process::exit(0);
        }
        let clean_save_data: BananenSaveDatav3 = BananenSaveDatav3 {
            main: (BananenSaveDataMain {
                bananen_version: _bananen_version.to_string(),
                bananendata_version: BANANEN_CONFIG_VERSION,
            }),
            config: (BananenConfig {
                changelogfile: String::from("changelog.md"),
                rollingrelease: false,
                customisation: BananenCustomisations {
                    log_name: String::from("Changelog"),
                    released_name: String::from("Releases"),
                    unreleased_name: String::from("Unreleased changes"),
                    changetypes: (Changetypes {
                        addition: (Changetype {
                            translation: String::from("Addition"),
                            color: String::from("#336600"),
                        }),
                        removal: (Changetype {
                            translation: String::from("Removal"),
                            color: String::from("#ff0000"),
                        }),
                        update: (Changetype {
                            translation: String::from("Update"),
                            color: String::from("#0033cc"),
                        }),
                        fix: (Changetype {
                            translation: String::from("Update"),
                            color: String::from("#9900cc"),
                        }),
                    }),
                },
            }),
            saved_changes: (BananensavedChanges {
                unreleased: [].to_vec(),
                released: [].to_vec(),
            }),
        };
        let clean_save_data_md =
            serde_json::to_string(&clean_save_data).expect("Could not create clean save data.");
        printer(
            0.2,
            format!("Writing new save data to '{color_cyan}{savefile}{color_reset}'!"),
        );
        to_savefile(clean_save_data_md.to_string());
        process::exit(0);
    }
    if !Path::new(&get_save_file_path()).exists() {
        if Path::new(&return_pathslashfile("bananen.toml")).exists() {
            printer(1.2,format!("{color_red}ERROR:{color_reset} This \"{0}\" is incompatible with this Bananen version, either reinit and manually update it, or use a different bananen version.",&return_pathslashfile("bananen.toml")));
            process::exit(1);
        }
        printer(1.3, format!(
                "{color_red}ERROR:{color_reset} No '{color_cyan}{savefile}{color_reset}' found. Use `{color_yellow}{me_bin}{color_reset} {color_blue}init{color_reset}` to create one."
            ));
        process::exit(1);
    }
    check_save_data_version();
    let mut _savedata = load_save_file();
    _savedata.main.bananen_version = VERSION.to_string();
    if command == "add" || command == "a" {
        if _a.is_empty() || _b.is_empty() {
            printer(1.4,format!(
                "{color_red}ERROR:{color_reset} No argument found for this {color_blue}add{color_reset}ition what do I need to add?"
            ));
            process::exit(1);
        }
        let additiontype: String = _a.clone();
        if additiontype == "add" || additiontype == "a" {
            printer(
                0.0,
                format!(
                    "(Accepted short `{0}` as `{1}`)",
                    additiontype,
                    _savedata
                        .config
                        .customisation
                        .changetypes
                        .addition
                        .translation
                ),
            );
        }
        let additiontype = if additiontype == "add" || additiontype == "a" {
            "addition"
        } else {
            &additiontype
        };
        if additiontype == "up" || additiontype == "u" {
            printer(
                0.0,
                format!(
                    "(Accepted short `{0}` as `{1}`)",
                    additiontype,
                    _savedata
                        .config
                        .customisation
                        .changetypes
                        .update
                        .translation
                ),
            );
        }
        let additiontype = if additiontype == "up" || additiontype == "u" {
            "update"
        } else {
            additiontype
        };
        if additiontype == "solve" || additiontype == "f" {
            printer(
                0.0,
                format!(
                    "(Accepted short `{0}` as `{1}`)",
                    additiontype, _savedata.config.customisation.changetypes.fix.translation
                ),
            );
        }
        let additiontype = if additiontype == "solve" || additiontype == "f" {
            "fix"
        } else {
            additiontype
        };
        if additiontype == "rem" || additiontype == "del" || additiontype == "r" {
            printer(
                0.0,
                format!(
                    "(Accepted short `{0}` as `{1}`)",
                    additiontype,
                    _savedata
                        .config
                        .customisation
                        .changetypes
                        .removal
                        .translation
                ),
            );
        }
        let additiontype = if additiontype == "rem" || additiontype == "del" || additiontype == "f"
        {
            "removal"
        } else {
            additiontype
        };
        if !(additiontype == "removal"
            || additiontype == "fix"
            || additiontype == "addition"
            || additiontype == "update")
        {
            printer(1.4,format!(
                "{color_red}ERROR:{color_reset} Incorrect type: `{additiontype}`! What kinda {color_blue}add{color_reset}ition is this?"
            ));
            process::exit(1);
        }
        let unedited_additiontype = &additiontype;
        let additiontype = if additiontype == "addition" {
            1
        } else if additiontype == "update" {
            2
        } else if additiontype == "fix" {
            3
        } else {
            4
        };
        let changelogfile = _savedata.config.changelogfile.clone();

        printer(
            0.1,
            format!(
                "{color_green}{0}{color_reset}: '{1}' --> unreleased@\"{2}\"",
                unedited_additiontype,
                _b,
                return_pathslashfile(&changelogfile)
            ),
        );
        let xychange: Change = Change {
            contents: _b,
            r#type: additiontype,
            breaking: _c == "--breaking",
        };
        let mut newchange = Vec::new();
        newchange.push(xychange);
        _savedata.saved_changes.unreleased.append(&mut newchange);
        let new_savedata_json =
            serde_json::to_string_pretty(&_savedata).expect("Error: Could not save data.");
        to_file(
            &generate_markdown_log(_savedata.saved_changes),
            return_pathslashfile(&changelogfile).as_str(),
        );
        to_savefile(new_savedata_json.to_string());
        process::exit(0);
    }
    if command == "regen" || command == "r" {
        let changelogfile = _savedata.config.changelogfile.clone();
        to_file(
            &generate_markdown_log(_savedata.saved_changes),
            return_pathslashfile(&changelogfile).as_str(),
        );
        printer(
            0.2,
            format!(
                "{color_green}Regenerated {color_reset}'{0}'!",
                return_pathslashfile(&changelogfile)
            ),
        );
        process::exit(0);
    }
    if command == "dub" || command == "d" || command == "push" || command == "bump" {
        if _a.is_empty() {
            printer(1.5,format!(
                "{color_red}ERROR:{color_reset} No release name found for this {color_blue}dub{color_reset}. Cannot log an unnamed release!"
            ));
            process::exit(1);
        }
        if _savedata.saved_changes.unreleased.is_empty() {
            printer(2.2,format!(
                "{color_red}ERROR:{color_reset} No release changes found for this {color_blue}dub{color_reset}. Cannot log a release without changes!"
            ));
            process::exit(1);
        }
        let releasename: String = _a.clone();
        let changelogfile = _savedata.config.changelogfile.clone();
        printer(
            0.3,
            format!(
                "unreleased@'{1}' --> {color_green}{0}{color_reset}@\"{1}\"",
                releasename,
                return_pathslashfile(&changelogfile)
            ),
        );
        let mut newreleases: Vec<ReleasedChanges> = [ReleasedChanges {
            name: releasename,
            changes: _savedata.saved_changes.unreleased,
        }]
        .to_vec();
        _savedata.saved_changes.unreleased = [].to_vec();
        newreleases.append(&mut _savedata.saved_changes.released);
        _savedata.saved_changes.released = newreleases;
        let new_savedata_json =
            serde_json::to_string_pretty(&_savedata).expect("Error: Could not save data.");
        to_file(
            &generate_markdown_log(_savedata.saved_changes),
            return_pathslashfile(&changelogfile).as_str(),
        );
        to_savefile(new_savedata_json.to_string());
        process::exit(0);
    }
    printer(1.1,format!(
                "{color_red}ERROR:{color_reset} Unknown command. Use `{color_yellow}{me_bin}{color_reset} {color_blue}help{color_reset}` for help."
            ));
    process::exit(1);
}

fn to_file(contents: &str, file: &str) {
    to_file2(contents, file)
        .map_err(|err| eprintln!("{:?}", err))
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
    contents
}
fn load_save_file() -> BananenSaveDatav3 {
    let savefile = &get_save_file_path();
    let unparsed_confi = from_file(savefile);
    let unparsed_config: &str = unparsed_confi.as_str();
    let me_bin = env::args().next().unwrap_or(String::from("unknown"));
    let expectationerror = format!("{color_red}ERROR:{color_reset} Expected I could understand '{color_cyan}{savefile}{color_reset}'!\nIf you don't mind resetting everything bananen has done, please reinitialise it with:\n`{color_yellow}{me_bin}{color_reset} {color_blue}init{color_reset} {color_black}{bg_white}--proceed{color_reset}{bg_reset}`.");
    serde_json::from_str(unparsed_config).expect(&expectationerror)
}
fn get_save_file_path() -> String {
    return_pathslashfile("bananen.json")
}
fn return_pathslashfile(file: &str) -> String {
    let expectationerror =
        format!("{color_red}ERROR:{color_reset} Expected a valid working directory.");
    let cd = env::current_dir().expect(&expectationerror);
    let sep = if cfg!(windows) { "\\" } else { "/" };
    let filed = if cfg!(windows) {
        file.replace('/', "\\")
    } else {
        file.replace('\\', "/")
    };
    format!("{0}{sep}{1}", cd.display(), filed)
}

fn check_save_data_version() {
    if Path::new(&return_pathslashfile("bananen.toml")).exists() {
        printer(2.3,format!("{color_red}ERROR:{color_reset} This configuration file ({0}) is incompatible with this Bananen version, either reinit and manually update it, or use a different bananen version.", return_pathslashfile("bananen.toml")));
        process::exit(1);
    }
    let savefile = &get_save_file_path();
    let unparsed_confi = from_file(savefile);
    let unparsed_config: &str = unparsed_confi.as_str();
    let me_bin = env::args().next().unwrap_or(String::from("unknown"));
    let expectationerror = format!("{color_red}ERROR:{color_reset} Expected I could understand '{color_cyan}{savefile}{color_reset}'!\nIf you don't mind resetting everything bananen has done, please reinitialise it with:\n`{color_yellow}{me_bin}{color_reset} {color_blue}init{color_reset} {color_black}{bg_white}--proceed{color_reset}{bg_reset}`.");
    let parsedsavefile: BasicBananenSaveData =
        serde_json::from_str(unparsed_config).expect(&expectationerror);
    if parsedsavefile.main.bananendata_version != BANANEN_CONFIG_VERSION {
        printer(2.3,format!("{color_red}ERROR:{color_reset} This \"{0}\" (v{1}) is incompatible with the Bananen savedata this installation supports (v{2}), either reinit and manually update it, or use a different bananen version.",
            savefile,
             parsedsavefile.main.bananendata_version,
             BANANEN_CONFIG_VERSION
        ));
        process::exit(1);
    }
}
fn generate_markdown_log(changes: BananensavedChanges) -> String {
    let localsavedata = load_save_file();
    let markdown: String;
    if !localsavedata.config.rollingrelease {
        let mut unreleasedchanges_md = String::from("No unreleased changes.");
        for change in changes.unreleased {
            let additiontype = change.r#type;
            let prespan = prespan(additiontype, &localsavedata);
            let breakingoption = if change.breaking {
                r#"<span style="color: red; background-color: #ffcc00">BREAKING!</span>"#
            } else {
                ""
            };
            if unreleasedchanges_md == "No unreleased changes." {
                unreleasedchanges_md = String::from("")
            };
            unreleasedchanges_md = format!(
                "- {3} {0}: {1}\n\r{2}",
                prespan, change.contents, unreleasedchanges_md, breakingoption
            );
        }
        let mut releasedchanges_md: String = "No releases yet.".to_string();
        for release in changes.released {
            if releasedchanges_md == "No releases yet." {
                releasedchanges_md = String::from("")
            };
            let mut release_md: String = String::from("");
            for change in &release.changes {
                let additiontype = change.r#type;
                let prespan = prespan(additiontype, &localsavedata);
                let breakingoption = if change.breaking {
                    r#"<span style="color: red; background-color: #ffcc00">BREAKING!</span>"#
                } else {
                    ""
                };
                release_md = format!(
                    "- {3} {0}: {1}\n\r{2}",
                    prespan, change.contents, release_md, breakingoption
                );
            }
            release_md = format!(
                r#"
### {0}
{1}"#,
                release.name, release_md
            );
            releasedchanges_md = format!("{1}\n\r{0}", release_md, releasedchanges_md)
        }
        let md: String = format!(
            r#"
# {1}


## {2}
            
{unreleasedchanges_md}

## {3}

{releasedchanges_md}

<hr>
            
This file was auto generated by [<span style="background-color: #24273a; color: #ffcc00">Bananen! 🍌</span>](https://github.com/strawmelonjuice/bananen/) `v{0}`
."#,
            localsavedata.main.bananen_version,
            localsavedata.config.customisation.log_name,
            localsavedata.config.customisation.unreleased_name,
            localsavedata.config.customisation.released_name
        );
        markdown = md;
    } else {
        let mut changes_md: String = "No changes.".to_string();
        for change in changes.unreleased {
            if changes_md == "No unreleased changes." {
                changes_md = String::from("")
            };
            let additiontype = change.r#type;
            let prespan = prespan(additiontype, &localsavedata);
            let breakingoption = if change.breaking {
                r#"<span style="color: red; background-color: #ffcc00">BREAKING!</span>"#
            } else {
                ""
            };
            changes_md = format!(
                "- {3} {0}: {1}\n\r{2}",
                prespan, change.contents, changes_md, breakingoption
            );
        }
        for release in changes.released {
            let mut release_md: String = String::from("");
            for change in &release.changes {
                if changes_md == "No unreleased changes." {
                    changes_md = String::from("")
                };
                let additiontype = change.r#type;
                let prespan = prespan(additiontype, &localsavedata);
                let breakingoption = if change.breaking {
                    r#"<span style="color: red; background-color: #ffcc00">BREAKING!</span>"#
                } else {
                    ""
                };
                release_md = format!(
                    "- {3} {0}: {1}\n\r{2}",
                    prespan, change.contents, release_md, breakingoption
                );
            }
            changes_md = format!("{1}\n\n{0}", release_md, changes_md)
        }
        let md: String = format!(
            r#"
# {1}


## {2}
            
{changes_md}

<hr>

This file was auto generated by [<span style="background-color: #24273a; color: #ffcc00">Bananen! 🍌</span>](https://github.com/strawmelonjuice/bananen/) `v{0}`."#,
            localsavedata.main.bananen_version,
            localsavedata.config.customisation.log_name,
            localsavedata.config.customisation.released_name,
        );
        markdown = md;
    }
    markdown
}
fn to_savefile(contents: String) {
    let savefile = &get_save_file_path();
    let bruh: &str = "{";
    let schematisation = format!(r#"{bruh}"$schema": "{0}", "main":"#, JSON_SCHEMA_URL);
    let annoyos: String = format!(r#"{bruh}"main":"#);
    let contents_schematised: String = contents.replacen(&annoyos, &schematisation, 1);
    to_file(&contents_schematised, savefile);
}

fn prespan(additiontype: i32, localsavedata: &BananenSaveDatav3) -> String {
    match additiontype {
        4 => {
            format!(
                r#"**<span style="color: {0}">{1}</span>**"#,
                localsavedata.config.customisation.changetypes.removal.color,
                localsavedata
                    .config
                    .customisation
                    .changetypes
                    .removal
                    .translation
            )
        }
        3 => {
            format!(
                r#"**<span style="color: {0}">{1}</span>**"#,
                localsavedata.config.customisation.changetypes.fix.color,
                localsavedata
                    .config
                    .customisation
                    .changetypes
                    .fix
                    .translation
            )
        }
        1 => {
            format!(
                r#"**<span style="color: {0}">{1}</span>**"#,
                localsavedata
                    .config
                    .customisation
                    .changetypes
                    .addition
                    .color,
                localsavedata
                    .config
                    .customisation
                    .changetypes
                    .addition
                    .translation
            )
        }
        2 => {
            format!(
                r#"**<span style="color: {0}">{1}</span>**"#,
                localsavedata.config.customisation.changetypes.update.color,
                localsavedata
                    .config
                    .customisation
                    .changetypes
                    .update
                    .translation
            )
        }
        _ => additiontype.to_string(),
    }
}
