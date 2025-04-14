// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;

fn main() {
     // Start Actix server
    backend::start_server();

    muzikfy_lib::run()
}
