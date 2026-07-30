#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod markdown;
mod note;
mod storage;

use dioxus::desktop::tao::dpi::LogicalSize;
use dioxus::desktop::{Config, WindowBuilder};

fn main() {
    let window = WindowBuilder::new()
        .with_title("Notes")
        .with_inner_size(LogicalSize::new(1180.0, 780.0))
        .with_min_inner_size(LogicalSize::new(820.0, 560.0));

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window))
        .launch(app::App);
}
