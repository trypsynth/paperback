use wxdragon::prelude::*;

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Paperback")
            .with_size(Size::new(800, 600))
            .build();

        frame.show(true);
        frame.centre();
    });
}
