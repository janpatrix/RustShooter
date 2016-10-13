extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

mod phi;
mod views;

fn main() {

    ::phi::spawn("Rusty Shooter", |phi| {
        Box::new(::views::game::ShipView::new(phi))
    });
}
 