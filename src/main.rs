use std::{error::Error, fs::read_to_string, path::PathBuf, process::Command as StdCommand};

use clap::{Arg, ArgAction, Command};
use console::Style;
use dialoguer::{theme::ColorfulTheme, Select};
use home::home_dir;

#[derive(serde::Deserialize)]
struct ConnectionList {
    name: String,
    connection: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let m = Command::new("sshs")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .value_name("FILE")
                .help("Specify a custom sshs.json file")
                .value_parser(clap::builder::PathBufValueParser::new()),
        )
        .get_matches();

    let path = match m.get_one::<PathBuf>("file") {
        Some(p) => p.to_owned(),
        None => default_sshs_path(),
    };

    let file = read_to_string(path)?;
    let connections: Vec<ConnectionList> = serde_json::from_str(file.as_str())?;

    let options: Vec<&str> = connections.iter().map(|c| &c.name[..]).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a connection")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    println!(
        "Connecting to {}",
        Style::new().cyan().apply_to(options[selection])
    );

    let conn = &connections[selection].connection;

    StdCommand::new("ssh")
        .arg(&conn)
        .spawn()
        .expect("Failed!")
        .wait()
        .expect("Failed to wait");

    Ok(())
}

fn default_sshs_path() -> PathBuf {
    let mut path = home_dir().expect("No home directory!");
    path.push(".ssh/sshs.json");
    path
}
