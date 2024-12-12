// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// set -gx WINIT_UNIX_BACKEND x11

fn main() {
    advent_calendar_lib::run()
}
