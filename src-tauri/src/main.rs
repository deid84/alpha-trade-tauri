// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

fn main() {
    dotenvy::dotenv().ok();

    let value = dotenvy::var("API_KEY").ok();
    println!("{}", value.unwrap_or("unknown?".to_string()));
    
    alpha_trade_tauri_lib::run()
}
