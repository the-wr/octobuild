extern crate octobuild;

use octobuild::clang::compiler::ClangCompiler;
use octobuild::compiler::*;
use octobuild::cluster::client::RemoteCompiler;
use octobuild::config::Config;

use std::env;
use std::io;
use std::io::{Error, Write};
use std::iter::FromIterator;
use std::path::Path;
use std::sync::Arc;
use std::process;

fn main() {
    process::exit(match compile() {
        Ok(status) => status,
        Err(e) => {
            println!("FATAL ERROR: {}", e);
            500
        }
    })
}

fn compile() -> Result<i32, Error> {
    let config = try!(Config::new());
    let args = Vec::from_iter(env::args());
    let state = Arc::new(SharedState::new(&config));
    let command_info = CommandInfo::simple(Path::new("clang"));
    let compiler = RemoteCompiler::new(&config.coordinator, ClangCompiler::new(), &state);
    let outputs = try!(compiler.compile(command_info, &args[1..], state.as_ref()));
    let mut status = 0;
    for output in outputs.into_iter() {
        try!(io::stdout().write_all(&output.stdout));
        try!(io::stderr().write_all(&output.stderr));
        if !output.success() {
            status = output.status.unwrap_or(501);
            break;
        }
    }
    println!("{}", state.statistic.to_string());
    Ok(status)
}
