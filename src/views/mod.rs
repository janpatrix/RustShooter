use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRect;

pub struct ShipView{
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0,
                }
            }
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64 ) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }
        //Move the Ship

        //Clear the Scene
        phi.renderer.set_draw_color(Color::RGB(0,0,0));
        phi.renderer.clear();

        //Render the scene
        phi.renderer.set_draw_color(Color::RGB(200, 50, 50));
        phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

        self.player.rect.x += 1.0;
        self.player.rect.y += 1.0;

        ViewAction::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64, 
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn to_sdl(self) -> Option<SdlRect> {
        assert!(self.w >= 0.0 && self.h >= 0.0);

        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32).unwrap()
    }
}

struct Ship {
    rect: Rectangle,
}


