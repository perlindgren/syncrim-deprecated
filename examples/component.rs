use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

fn main() {
    Application::new(|cx| {
        // Element::new(cx)
        //     .width(Pixels(100.0))
        //     .height(Pixels(100.0))
        //     .border_color(Color::black())
        //     .border_width(Pixels(1.0));
        let m1 = Mux::new(cx, 200.0, 100.0, 3);
        let w1 = Wire::new(cx, 100.0, 100.0, 200.0, 100.0);
        let w2 = Wire::new(cx, 100.0, 100.0, 100.0, 200.0);
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

impl Model for Port {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {}
}

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

        path.move_to(bounds.left(), bounds.top());
        path.line_to(bounds.right(), bounds.top());
        path.line_to(bounds.right(), bounds.bottom());
        path.line_to(bounds.left(), bounds.bottom());
        path.line_to(bounds.left(), bounds.top());

        canvas.stroke_path(&mut path, &paint);
    }
}

#[derive(Lens, Clone)]
pub struct Mux {
    //out: Port,
}

impl Model for Mux {}

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
                // // inputs
                // for i in 0..nr_in {
                //     Port::new(cx, 0.0, (i + 1) as f32 * MUX_SPACE);
                // }

                // Port::new(cx, half_width, 0.0);
                // Port::new(cx, half_width * 2.0, half_hight);
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

        path.move_to(bounds.left(), bounds.top());
        path.line_to(bounds.right(), bounds.top());
        path.line_to(bounds.right(), bounds.bottom());
        path.line_to(bounds.left(), bounds.bottom());
        path.line_to(bounds.left(), bounds.top());

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

impl Model for Wire {}

impl Wire {
    pub fn new<'a>(cx: &'a mut Context, x1: f32, y1: f32, x2: f32, y2: f32) -> Handle<'a, Self> {
        let direction = if y1 == y2 {
            Direction::Horizontal
        } else {
            Direction::Vertical
        };
        let handle = vizia::prelude::View::build(Self { direction }, cx, |cx| {})
            .position_type(PositionType::SelfDirected)
            .left(Pixels(x1))
            .top(Pixels(y1 - 2.0));

        match direction {
            Direction::Horizontal => handle.width(Pixels(x2 - x1)).height(Pixels(4.0)),
            Direction::Vertical => handle.width(Pixels(4.0)).height(Pixels(y2 - y1)),
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
        path.move_to(bounds.left(), bounds.top());
        path.line_to(bounds.right(), bounds.top());
        path.line_to(bounds.right(), bounds.bottom());
        path.line_to(bounds.left(), bounds.bottom());
        path.line_to(bounds.left(), bounds.top());

        canvas.stroke_path(&mut path, &paint);

        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 1.0));
        paint.set_line_width(cx.logical_to_physical(1.0));
        let mut path = Path::new();
        match self.direction {
            Direction::Horizontal => {
                path.move_to(bounds.left(), bounds.top() + bounds.height() * 0.5);
                path.line_to(bounds.right(), bounds.top() + bounds.height() * 0.5);
            }
            Direction::Vertical => {
                path.move_to(bounds.left() + bounds.width() * 0.5, bounds.top());
                path.line_to(bounds.left() + bounds.width() * 0.5, bounds.bottom());
            }
        };
        canvas.stroke_path(&mut path, &paint);
    }
}

// #[derive(Lens, Clone, Debug)]
// pub struct Port<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     data: T,
// }

// impl<T> Display for Port<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.data)
//     }
// }

// #[derive(Debug)]
// pub enum PortEvent<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     SetData(T),
// }

// impl<T> Model for Port<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     fn event(&mut self, _: &mut EventContext, event: &mut Event) {
//         // event.map(|port_event, _| match app_event {
//         //     AppEvent::ShowModal => {
//         //         self.show_modal = true;
//         //     }
//         //     AppEvent::HideModal => {
//         //         self.show_modal = false;
//         //     }
//         // });
//     }
// }

// impl<T> Port<T>
// where
//     T: Display + Clone + 'static + Debug,
// {
//     pub fn new<'a, A, L>(
//         cx: &'a mut Context,
//         action: A,
//         lens: L,
//         name: &str,
//         data: T,
//     ) -> Handle<'a, Self>
//     where
//         A: 'static + Fn(&mut EventContext),
//         L: Lens<Target = Port<T>>,
//     {
//         // Self { data }.build(cx, |cx| {
//         //     Label::new(cx, MINUS).class("icon");
//         // })
//     }
// }

// impl<T> View for Port<T> where T: Display + Clone + 'static + Debug {}
