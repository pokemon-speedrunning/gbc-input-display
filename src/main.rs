#![windows_subsystem = "windows"]

mod application;
mod bmp;
mod configuration;
mod dpad;
mod gambatte;
mod gfx;
mod key;
mod platform;
mod winapi;

fn main() {
    application::start();
}
