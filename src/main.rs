extern crate toml;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::process::Command;


use toml::Value;
use toml::value::Table;

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

fn extract_section(section_name: &str, master_table: Value) -> Value {
    match master_table.get(section_name) {
        Some(value) => value.clone(),
        None => panic!("No such key {} found in master table!", section_name),
    }
}

fn main() {
    // Grab all the whole Toml file
    let master_table = read_toml("./Packages.toml");
    let dependencies = extract_section("dependencies", master_table);
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

    // Now attempt to install with apt
    let output = Command::new("apt")
        .arg("-y")
        .arg("install")
        .arg(dep_vec.join(" "))
        .output()
        .expect("Process failed to execute")
        .stdout;

    // Let the user see the output
    println!("{}", String::from_utf8_lossy(&output));
}
