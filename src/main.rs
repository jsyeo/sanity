extern crate yaml_rust;
extern crate ansi_term;

use ansi_term::Colour::{Red, Cyan};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use std::io::prelude::*;
use yaml_rust::{YamlLoader};

const CONFIG_PATH: &'static str = ".sanity.yml";
const EMPTY_CONFIG: &'static str =
"\
# Define commands and phases to be ran as part of your project's CI/CD workflow. Currently,\n\
# there are 6 phases: install, syntax, lint, unit, functional, and security. sanity will run\n\
# them in that order and fails if any of the commands fails. \n\
#\n\
# Ruby project example:\n\
#\n\
# install: bundle install\n\
# syntax:\n\
#   - rubocop\n\
#   - reek\n\
# unit: rspec\n\
# security: brakeman\n\
\n\
install: \n\
syntax:\n\
lint:\n\
unit:\n\
functional:\n\
security:\n\
";

fn main() {
    let path = Path::new(CONFIG_PATH);
    let mut args = env::args();
    args.next();
    let second = args.next();
    if second.is_some() {
        let arg = second.unwrap();
        match arg.as_str() {
            "init" => create_sanity_config(),
            _      => println!("{}", Red.paint(format!("Unknown command {}.", arg))),
        }
        exit(0);
    }

    if !path.exists() {
        println!("{}", Red.paint(format!("{} does not exist, try running `sanity init`", CONFIG_PATH)));
        exit(1);
    }

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {:?} because {}", &path, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s);

    let docs = match YamlLoader::load_from_str(&s[..]) {
        Err(why) => panic!("Couldn't parse {:?} because {}", &path, why.description()),
        Ok(docs) => docs,
    };

    let doc = &docs[0];

    let phases = ["install", "syntax", "lint", "unit", "functional", "security"];

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

fn create_sanity_config() {
    let path = Path::new(CONFIG_PATH);
    if path.exists() {
        println!("{}", Red.paint(format!("{:?} already exists.", path)));
    } else {
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {:?}: {}", path, why.description()),
            Ok(file) => file,
        };

        match file.write_all(EMPTY_CONFIG.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {:?}: {}", path,
                       why.description())
            },
            Ok(_) => println!("{}", Cyan.paint(format!("Created an empty {}", CONFIG_PATH))),
        }
    }
}
