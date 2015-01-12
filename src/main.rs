#![feature(plugin)]
#![allow(unstable)]

extern crate libc;
extern crate getopts;
extern crate toml;
#[plugin]
extern crate regex_macros;
extern crate regex;
extern crate time;

mod settings;
mod process;
mod monitor;

fn main() {
    let settings = match settings::get_settings() {
        None => { return; },
        Some(s) => { s }
    };
    process::set_pid_file(settings.pid_file_path());
    println!("{:?}", monitor::monitor_stat());
    process::remove_pid_file(settings.pid_file_path());
}

