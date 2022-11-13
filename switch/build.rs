#![feature(exit_status_error)]
use std::process::{ExitStatusError};

use npm_rs::*;

fn main() -> (){
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../web/build/asset-manifest.json");
    
   NpmEnv::default()
        .set_path("../web")
        .init_env()
        .install(None)
        .run("build")
        .exec().unwrap()
        .exit_ok().unwrap();
}