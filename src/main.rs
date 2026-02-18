mod components;
mod markdown;
mod state;
mod storage;

use components::App;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    LaunchBuilder::new()
        .with_cfg(desktop! {
            use dioxus::desktop::{Config, WindowBuilder};
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Mime")
                    .with_maximized(true),
            )
        })
        .launch(|| {
            rsx! {
                document::Title { "Mime" }
                document::Link { rel: "stylesheet", href: MAIN_CSS }
                App {}
            }
        });
}
