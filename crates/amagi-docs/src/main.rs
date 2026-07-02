mod app;
mod build_info;
mod components;
mod docs_index;
mod markdown;
mod models;
mod search;
mod storage;
mod theme;

fn main() {
    dioxus::prelude::launch(app::App);
}
