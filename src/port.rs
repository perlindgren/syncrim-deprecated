use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Lens, Clone)]
pub struct Port {}

#[derive(Debug)]
pub enum PortEvent {
    SetData,
}

pub const PORT_SIZE: f32 = 10.0;

// impl Model for Port {
//     fn event(&mut self, _: &mut EventContext, event: &mut Event) {}
// }

impl Port {
    pub fn new<'a>(cx: &'a mut Context, x: f32, y: f32) -> Handle<'a, Self> {
        vizia::prelude::View::build(Self {}, cx, |cx| {})
            .position_type(PositionType::SelfDirected)
            .left(Pixels(x - PORT_SIZE * 0.5))
            .top(Pixels(y - PORT_SIZE * 0.5))
            .width(Pixels(PORT_SIZE))
            .height(Pixels(PORT_SIZE))
    }
}

impl View for Port {
    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("port draw {:?}", bounds);

        let mut path = Path::new();

        let mut paint = Paint::color(vizia::vg::Color::black());
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.stroke_path(&mut path, &paint);
    }
}
