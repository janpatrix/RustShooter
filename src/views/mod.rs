use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;

pub struct DefaultView;
pub struct ViewA;
pub struct ViewB;

impl View for DefaultView {
    fn render(&mut self, contex: &mut Phi, _: f64) -> ViewAction { 
        let renderer = &mut contex.renderer;
        let events = &mut contex.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewA));
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        ViewAction::None
    }
}

impl View for ViewA {
    fn render(&mut self, contex: &mut Phi, _: f64) -> ViewAction { 
        let renderer = &mut contex.renderer;
        let events = &mut contex.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewB));
        }

        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.clear();

        ViewAction::None
    }
}

impl View for ViewB {
    fn render(&mut self, contex: &mut Phi, _: f64) -> ViewAction { 
        let renderer = &mut contex.renderer;
        let events = &mut contex.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(DefaultView));
        }

        renderer.set_draw_color(Color::RGB(255, 255, 0));
        renderer.clear();

        ViewAction::None
    }
}