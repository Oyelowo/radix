mod app;
mod primitives;
mod use_node_ref;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    dioxus::launch(App)
}
