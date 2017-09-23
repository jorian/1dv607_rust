mod model;
mod controller;
mod view;

use view::console;

fn main() {
    let ui = console::Console { };
    ui.start_program();
}
