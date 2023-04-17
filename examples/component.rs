use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Lens, Debug)]
struct SimState {
    pc_reg: u32,
}

fn main() {
    Application::new(|cx| {
        // just to check layout of Vizia components
        // Element::new(cx)
        //     .top(Pixels(100.0))
        //     .left(Pixels(100.0))
        //     .width(Pixels(100.0))
        //     .height(Pixels(100.0))
        //     .border_color(Color::black())
        //     .border_width(Pixels(1.0));

        // // Custom components
        // let _m1 = Mux::new(cx, 200.0, 100.0, 3);

        // let _p1 = Port::new(cx, 50.0, 100.0);

        // let _p2 = Port::new(cx, 50.0, 120.0);

        // let _w1 = Wire::new(cx, 50.0, 100.0, 180.0, 100.0);

        // let _w1 = Wire::new(cx, 50.0, 120.0, 180.0, 120.0);

        // let _m1 = Mux::new(cx, 200.0, 400.0, 2);

        // let _p1 = Port::new(cx, 50.0, 390.0);

        // let _p2 = Port::new(cx, 50.0, 410.0);

        // let _w1 = Wire::new(cx, 50.0, 390.0, 180.0, 390.0);

        // let _w1 = Wire::new(cx, 50.0, 410.0, 180.0, 410.0);

        let _sync = Synchronizer::new(cx, 200.0, 200.0);
        let _w1 = Wire::new(cx, 205.0, 200.0, 230.0, 200.0);
        let _w1 = Wire::new(cx, 230.0, 200.0, 230.0, 220.0);
        let _w1 = Wire::new(cx, 230.0, 220.0, 170.0, 220.0);
        let _w1 = Wire::new(cx, 170.0, 220.0, 170.0, 200.0);
        let _w1 = Wire::new(cx, 170.0, 200.0, 195.0, 200.0);
    })
    .run();
}

#[derive(Lens, Clone)]
pub struct Port {}

#[derive(Debug)]
pub enum PortEvent {
    SetData,
}

const PORT_SIZE: f32 = 10.0;

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

#[derive(Lens, Clone)]
pub struct Mux {
    //  out: (f32, f32),
}

// impl Model for Mux {}

const MUX_WIDTH: f32 = 40.0;
const MUX_SPACE: f32 = 20.0;

impl Mux {
    pub fn new<'a>(cx: &'a mut Context, x: f32, y: f32, nr_in: usize) -> Handle<'a, Self> {
        let half_width = MUX_WIDTH * 0.5;
        let half_hight = (1 + nr_in) as f32 * MUX_SPACE * 0.5;
        vizia::prelude::View::build(
            Self {
                // out: Port::new(cx, 0.0, (i + 1) as f32 * MUX_SPACE),
            },
            cx,
            |cx| {
                // inputs
                for i in 0..nr_in {
                    Port::new(cx, 0.0, (i + 1) as f32 * MUX_SPACE);
                }

                Port::new(cx, half_width, 0.0);
                // output
                Port::new(cx, half_width * 2.0, half_hight);
            },
        )
        .position_type(PositionType::SelfDirected)
        .left(Pixels(x - half_width))
        .top(Pixels(y - half_hight))
        .width(Pixels(MUX_WIDTH))
        .height(Pixels(half_hight * 2.0))
    }
}

impl View for Mux {
    fn element(&self) -> Option<&'static str> {
        Some("mux")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("mux draw {:?}", bounds);

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

#[derive(Lens, Clone)]
pub struct Synchronizer {}

// impl Model for Synchronizer {}

impl Synchronizer {
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

impl View for Synchronizer {
    fn element(&self) -> Option<&'static str> {
        Some("synchronizer")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("synchronizer draw {:?}", bounds);

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

#[derive(Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}
#[derive(Lens, Clone)]
pub struct Wire {
    direction: Direction,
}

// impl Model for Wire {}

const WIRE_WIDTH: f32 = 4.0;

impl Wire {
    pub fn new<'a>(cx: &'a mut Context, x1: f32, y1: f32, x2: f32, y2: f32) -> Handle<'a, Self> {
        let direction = if y1 == y2 {
            Direction::Horizontal
        } else {
            Direction::Vertical
        };
        let handle = vizia::prelude::View::build(Self { direction }, cx, |cx| {})
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
