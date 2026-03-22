// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::prelude::*;
use std::process::{}

fn main() {
    qemu_cheater_lib::run()
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn command_run(command_option){
    let mut child = Command:new("qemu-system-x86_64")
        .arg(command_option)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");
}
