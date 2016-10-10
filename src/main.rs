extern crate sdl2;

mod phi;
mod views;

fn main() {

    ::phi::spawn("Rusty Shooter", |_| {
        Box::new(::views::DefaultView)
    });
}
 