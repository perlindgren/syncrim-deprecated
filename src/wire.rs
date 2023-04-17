use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

// #[derive(Lens, Clone)]
pub struct Wire {
    direction: Direction,
}

const WIRE_WIDTH: f32 = 4.0;

impl Wire {
    pub fn new<'a>(cx: &'a mut Context, x1: f32, y1: f32, x2: f32, y2: f32) -> Handle<'a, Self> {
        let direction = if y1 == y2 {
            Direction::Horizontal
        } else {
            Direction::Vertical
        };
        let handle = vizia::prelude::View::build(Self { direction }, cx, |_cx| {})
            .position_type(PositionType::SelfDirected)
            .left(Pixels(f32::min(x1, x2) - WIRE_WIDTH * 0.5))
            .top(Pixels(f32::min(y1, y2) - WIRE_WIDTH * 0.5));

        match direction {
            Direction::Horizontal => handle
                .width(Pixels(f32::abs(x2 - x1) + WIRE_WIDTH))
                .height(Pixels(WIRE_WIDTH)),
            Direction::Vertical => handle
                .width(Pixels(WIRE_WIDTH))
                .height(Pixels(f32::abs(y2 - y1) + WIRE_WIDTH)),
        }
    }
}

impl View for Wire {
    fn element(&self) -> Option<&'static str> {
        Some("wire")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        println!("wire bounds {:?}", bounds);

        let mut path = Path::new();

        let mut paint = Paint::color(vizia::vg::Color::black());
        paint.set_line_width(cx.logical_to_physical(1.0));
        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.stroke_path(&mut path, &paint);

        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 1.0));
        paint.set_line_width(cx.logical_to_physical(1.0));
        let mut path = Path::new();
        match self.direction {
            Direction::Horizontal => {
                path.move_to(
                    bounds.left() + bounds.height() * 0.5 + 0.5,
                    bounds.top() + bounds.height() * 0.5 + 0.5,
                );
                path.line_to(
                    bounds.right() - bounds.height() * 0.5 + 0.5,
                    bounds.top() + bounds.height() * 0.5 + 0.5,
                );
            }
            Direction::Vertical => {
                path.move_to(
                    bounds.left() + bounds.width() * 0.5 + 0.5,
                    bounds.top() + bounds.width() * 0.5 + 0.5,
                );
                path.line_to(
                    bounds.left() + bounds.width() * 0.5 + 0.5,
                    bounds.bottom() - bounds.width() * 0.5 + 0.5,
                );
            }
        };
        canvas.stroke_path(&mut path, &paint);
    }
}
