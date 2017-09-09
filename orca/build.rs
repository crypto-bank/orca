extern crate glob;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate protoc;
extern crate protoc_rust;

use std::fs;
use std::env;
use std::path::Path;


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
        .collect()
}


fn clean_old_files() {
    for f in glob_simple("core/**/*_pb.rs") {
        fs::remove_file(f).expect("rm");
    }
}

fn compile_dir(dir: &str, out: &str) {
    let protos = glob_simple(dir);
    assert!(!protos.is_empty());
    let input = protos.iter().map(|a| format!("orca/{}", a)).collect::<Vec<String>>();

    let root = Path::new("../");
    assert!(env::set_current_dir(&root).is_ok());

    protoc_rust::run(protoc_rust::Args {
        out_dir: out,
        input: &input.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
        includes: &["."],
        // input: &protos.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
    }).expect("protoc");

    warn!("Generated!");

    let root = Path::new("./orca");
    assert!(env::set_current_dir(&root).is_ok());
}

fn main() {
    env_logger::init().expect("env_logger");
    println!("cargo:rerun-if-changed=build.rs");

    warn!("Generating protocol buffers code.");

    clean_old_files();

    compile_dir("core/*.proto", "./orca/core");
    compile_dir("core/client/*.proto", "./orca/core/client");

    if protoc::Protoc::from_env_path()
        .version()
        .expect("version")
        .is_3()
    {
        println!("cargo:rustc-cfg=proto3");
    }
}