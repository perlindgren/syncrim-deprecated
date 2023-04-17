use syncrim::{register::Register, wire::Wire};
use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Lens, Debug)]
struct SimState {
    pc_reg: u32,
}

fn main() {
    Application::new(|cx| {
        let _sync = Register::new(cx, 200.0, 200.0);
        let _w1 = Wire::new(cx, 205.0, 200.0, 230.0, 200.0);
        let _w1 = Wire::new(cx, 230.0, 200.0, 230.0, 220.0);
        let _w1 = Wire::new(cx, 230.0, 220.0, 170.0, 220.0);
        let _w1 = Wire::new(cx, 170.0, 220.0, 170.0, 200.0);
        let _w1 = Wire::new(cx, 170.0, 200.0, 195.0, 200.0);
    })
    .run();
}
