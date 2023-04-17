use crate::port::{Port, PORT_SIZE};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Lens, Clone)]
pub struct RegView<L: Lens> {
    lens: L,
}

impl<L: Lens> RegView<L>
where
    <L as Lens>::Target: Data + Clone + ToString,
{
    pub fn new<'a>(cx: &'a mut Context, lens: L, x: f32, y: f32) -> Handle<'a, Self> {
        let half_width = PORT_SIZE;
        let half_hight = PORT_SIZE * 0.5;

        vizia::prelude::View::build(Self { lens: lens.clone() }, cx, |cx| {
            // input
            Port::new(cx, half_width * 0.5, half_hight);
            // output
            Port::new(cx, half_width * 1.5, half_hight);
            Label::new(cx, lens);
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(x - half_width))
        .top(Pixels(y - half_hight))
        .width(Pixels(half_width * 2.0))
        .height(Pixels(half_hight * 2.0))
    }
}

impl<L> View for RegView<L>
where
    L: Lens,
{
    fn element(&self) -> Option<&'static str> {
        Some("RegView")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("Register draw {:?}", bounds);
        // println!("-- for now just printing state -- {:?}", self.data.view());

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
