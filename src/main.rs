extern crate yaml_rust;
extern crate ansi_term;

use ansi_term::Colour::{Red, Cyan};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use std::io::Read;
use yaml_rust::{YamlLoader};

fn main() {
    let path = Path::new(".sanity.yml");

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {:?} because {}", path, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s);

    let docs = match YamlLoader::load_from_str(&s[..]) {
        Err(why) => panic!("Couldn't parse {:?} because {}", path, why.description()),
        Ok(docs) => docs,
    };

    let doc = &docs[0];

    let phases = ["syntax", "lint", "unit", "functional", "security"];

    for &phase in phases.iter() {
        let value = &doc[phase];
        if value.is_null() || value.is_badvalue() {
            continue;
        }

        println!("{}", Cyan.bold().paint(format!("Running {} phase...", phase)));
        let commands = match *value {
            yaml_rust::yaml::Yaml::String(ref s) => vec!(s.as_str()),
            yaml_rust::yaml::Yaml::Array(ref a) => a.iter().map(|x| x.as_str().unwrap()).collect(),
            _ => panic!("String expected for {} phase, but got {:?}", phase, value),
        };

        for full_cmd in commands.iter() {
            let mut iter = full_cmd.split_whitespace();
            let c = iter.next().unwrap();

            let args = &iter.collect::<Vec<&str>>();
            let status = match Command::new(c).args(args).status() {
                Err(why) => panic!("Unable to execute command: {} because {}", full_cmd, why.description()),
                Ok(status) => status,
            };

            if !status.success() {
                let msg = match status.code() {
                    None => format!("{} killed by a signal", c),
                    Some(code) => format!("{} failed with exit code {}", c, code),
                };
                println!("{}", Red.paint(msg));
                exit(1);
            }
        }
    }
}
