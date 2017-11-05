#![feature(rustc_private)]

extern crate glob;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate protoc;
extern crate protoc_rust;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

const GENERATED_FILES: &'static [&'static str] = &[
    "orca/currency/currency.rs",
    "orca/events/events.rs",
    "orca/markets/markets.rs",
];

fn glob_simple(pattern: &str) -> Vec<String> {
    glob::glob(pattern)
        .expect("glob")
        .map(|g| {
            g.expect("item")
                .as_path()
                .to_str()
                .expect("utf-8")
                .to_owned()
        })
        .map(|p| p.replace("\\", "/"))
        .collect()
}

fn clean_old_files() {
    for f in GENERATED_FILES {
        if let Err(e) = fs::remove_file(f) {
            warn!("couldn't remote {}, err: {:?}", f, e);
        }
    }
}

fn compile_dir(dir: &str, out: &str) {
    let protos = glob_simple(dir);
    assert!(!protos.is_empty());

    protoc_rust::run(protoc_rust::Args {
        out_dir: out,
        includes: &["."],
        input: &protos.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
    }).expect("protoc");

    println!("Generated!");
}

fn main() {
    env_logger::init().expect("env_logger");
    println!("cargo:rerun-if-changed=build.rs");

    warn!("Generating protocol buffers code.");

    clean_old_files();

    compile_dir("orca/currency/*.proto", "./orca/currency");
    compile_dir("orca/events/*.proto", "./orca/events");
    compile_dir("orca/markets/*.proto", "./orca/markets");

    // Read generated files and
    // replace all occurences of:
    //  1. `super::events` to `::events`.
    //  2. `super::markets` to `::markets`.
    //  3. `super::currency` to `::currency`.
    for filename in GENERATED_FILES {
        // read file to string
        let mut contents = String::new();
        {
            let mut f = File::open(filename)
                .expect(&format!("file not found: {}", filename));
            f.read_to_string(&mut contents).expect("read file");
        }
        // replace occurences
        contents = contents.replace("super::events", "::events");
        contents = contents.replace("super::markets", "::markets");
        contents = contents.replace("super::currency", "::currency");
        // create file
        let mut file = File::create(filename)
            .expect("create file");
        // write back to file
        file.write_all(contents.as_bytes())
            .expect("write to file");
    }

    if protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3()
    {
        println!("cargo:rustc-cfg=proto3");
    }
}
