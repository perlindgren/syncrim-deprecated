// use crate::common::*;
use crate::port::{Port, PORT_SIZE};
use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Lens, Clone)]
pub struct Register {}

// impl Model for Register {}

impl Register {
    pub fn new<'a>(cx: &'a mut Context, x: f32, y: f32) -> Handle<'a, Self> {
        let half_width = PORT_SIZE;
        let half_hight = PORT_SIZE * 0.5;
        vizia::prelude::View::build(Self {}, cx, |cx| {
            // input
            Port::new(cx, half_width * 0.5, half_hight);
            // output
            Port::new(cx, half_width * 1.5, half_hight);
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(x - half_width))
        .top(Pixels(y - half_hight))
        .width(Pixels(half_width * 2.0))
        .height(Pixels(half_hight * 2.0))
    }
}

impl View for Register {
    fn element(&self) -> Option<&'static str> {
        Some("Register")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("Register draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.stroke_path(&mut path, &paint);
    }
}
