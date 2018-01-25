extern crate toml;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::process::Command;

use toml::Value;
use std::process::Stdio;

pub mod data_types;

fn read_toml(path: &str) -> Value {
    let mut file = match File::open(path) {
        Err(e) => panic!("Could not open {}: {}", path, e.description()),
        Ok(f) => f,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("Could not read {}: {}", path, e.description()),
        Ok(_) => (),
    }

    s.parse::<Value>().unwrap()
}

fn extract_section(section_name: &str, master_table: &Value) -> Value {
    match master_table.get(section_name) {
        Some(value) => value.clone(),
        None => panic!("No such key {} found in master table!", section_name),
    }
}

/// If the user has specified that they're using apt, we execute apt specific
/// commands, like installing PPAs.
fn apt_install(dependencies: Vec<String>, master_table: &Value) -> Command {
    // PPA phase
    let ppas = extract_section("ppas", master_table);

    // Now attempt to install with apt
    let mut command = Command::new("apt");
    command.arg("-y").arg("install");
    for arg in dependencies {
        command.arg(arg);
    }
    command
}

fn main() {
    // Grab all the whole Toml file
    let master_table = read_toml("./Packages.toml");

    let config = extract_section("config", &master_table);
    let platform = config.get("platform").unwrap().to_string();

    // Dependency Phase
    let dependencies = extract_section("dependencies", &master_table);
    let mut dep_vec: Vec<String> = Vec::new();

    // Iterate over the dependencies section
    for (dep, ver) in dependencies
        .try_into::<BTreeMap<String, String>>()
        .unwrap()
        .into_iter()
    {
        // Push an apt-readable package name to the dependency vec
        dep_vec.push(match ver.as_ref() {
            "*" => dep,
            _ => format!("{}={}", dep, ver)
        })
    }

    let mut command = match platform.as_ref() {
        "apt" => apt_install(dep_vec, &master_table),
        _ => Command::new("anything")
    };

    let result = command.stdout(Stdio::inherit()).spawn()
        .unwrap()
        .wait()
        .unwrap();
    println!("All done!");
}
