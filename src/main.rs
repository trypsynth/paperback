#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines)]

mod ui;

use ui::MainWindow;

fn main() {
    let _ = wxdragon::main(|_| {
        let main_window = MainWindow::new();
        main_window.show();
    });
}
