use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
use vizia::vg::{LineCap, Paint, Path, Solidity};

// const STYLE: &str = r#"

//     .modal {
//         space: 1s;
//         background-color: white;
//         border-radius: 3px;
//         border-width: 1px;
//         border-color: #999999;
//         outer-shadow: 0 3 10 #00000055;
//         overflow: visible;
//         child-space: 10px;
//     }

//     modal>popup>label {
//         width: auto;
//         height: auto;
//         space: 5px;
//         child-space: 1s;
//     }

//     button {
//         border-radius: 3px;
//         child-space: 1s;
//     }

//     hstack {
//         child-space: 1s;
//         col-between: 20px;
//     }
// "#;
// cx.add_theme(STYLE);

// AppData { show_modal: false }.build(cx);

// let m1 = Mux::new(cx, 200.0, 200.0);
// let m2 = Mux::new(cx, 300.0, 200.0);
fn main() {
    Application::new(|cx| {
        Element::new(cx)
            .width(Pixels(100.0))
            .height(Pixels(100.0))
            .border_color(Color::black())
            .border_width(Pixels(1.0));
        let w1 = Wire::new(cx, 10.0, 100.0, 20.0, 100.0);
    })
    .run();
}

// #[derive(Debug)]
// pub enum AppEvent {
//     ShowModal,
//     HideModal,
// }

// #[derive(Lens)]
// pub struct AppData {
//     show_modal: bool,
// }

// impl Model for AppData {
//     fn event(&mut self, _: &mut EventContext, event: &mut Event) {
//         event.map(|app_event, _| match app_event {
//             AppEvent::ShowModal => {
//                 self.show_modal = true;
//             }
//             AppEvent::HideModal => {
//                 self.show_modal = false;
//             }
//         });
//     }
// }

// use std::fmt::{self, Debug, Display};

// #[derive(Lens, Clone)]
// pub struct Port {}

// #[derive(Debug)]
// pub enum PortEvent {
//     SetData,
// }

// impl Model for Port {
//     fn event(&mut self, _: &mut EventContext, event: &mut Event) {}
// }

// impl Port {
//     pub fn new<'a>(cx: &'a mut Context) -> Handle<'a, Self> {
//         vizia::prelude::View::build(Self {}, cx, |cx| {
//             // Label::new(cx, MINUS).class("icon");
//         })
//         .width(Pixels(10.0))
//         .height(Pixels(10.0))
//     }
// }

// impl View for Port {
//     fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
//         let bounds = cx.bounds();

//         let mut path = Path::new();

//         let mut paint = Paint::color(Color::black());
//         paint.set_line_width(1.0);
//         // paint.set_line_cap(LineCap::Round);

//         path.move_to(bounds.left(), bounds.top());
//         path.line_to(bounds.right(), bounds.top());
//         path.line_to(bounds.right(), bounds.bottom());
//         path.line_to(bounds.left(), bounds.bottom());
//         path.line_to(bounds.left(), bounds.top());

//         canvas.stroke_path(&mut path, &paint);
//     }
// }

// #[derive(Lens, Clone)]
// pub struct Mux {}

// impl Model for Mux {}

// impl Mux {
//     pub fn new<'a>(cx: &'a mut Context, x: f32, y: f32) -> Handle<'a, Self> {
//         vizia::prelude::View::build(Self {}, cx, |cx| {
//             // left side
//             for i in 0..10 {
//                 Port::new(cx)
//                     // .width(Pixels(150.0))
//                     .position_type(PositionType::SelfDirected)
//                     .top(Pixels(20.0 * i as f32))
//                     .left(Pixels(0.0));
//             }

//             Port::new(cx)
//                 // .width(Pixels(150.0))
//                 .position_type(PositionType::SelfDirected)
//                 .top(Pixels(100.0))
//                 .left(Pixels(40.0));
//         })
//         .position_type(PositionType::SelfDirected)
//         .left(Pixels(x - 20.0))
//         .top(Pixels(y - 100.0))
//         .width(Pixels(40.0))
//         .height(Pixels(200.0))

//         // .height(Pixels(100.0))
//     }
// }

// impl View for Mux {
//     fn element(&self) -> Option<&'static str> {
//         Some("mux")
//     }

//     fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
//         println!("draw");

//         let bounds = cx.bounds();

//         let mut path = Path::new();

//         let mut paint = Paint::color(Color::black());
//         // paint.set_line_width(line_width);
//         // paint.set_line_cap(LineCap::Round);

//         path.move_to(bounds.left(), bounds.top());
//         path.line_to(bounds.right(), bounds.top());
//         path.line_to(bounds.right(), bounds.bottom());
//         path.line_to(bounds.left(), bounds.bottom());
//         path.line_to(bounds.left(), bounds.top());

//         canvas.stroke_path(&mut path, &paint);
//     }
// }

#[derive(Lens, Clone)]
pub struct Wire {
    from: (f32, f32),
    to: (f32, f32),
}

impl Model for Wire {}

impl Wire {
    pub fn new<'a>(cx: &'a mut Context, x1: f32, y1: f32, x2: f32, y2: f32) -> Handle<'a, Self> {
        vizia::prelude::View::build(
            Self {
                from: (x1, y1),
                to: (x2, y2),
            },
            cx,
            |cx| {},
        )
        // .position_type(PositionType::SelfDirected)
        // .left(Pixels(x1))
        // .top(Pixels(y1 - 1.0))
        // .width(Pixels(x2 - x1))
        // .height(Pixels(3.0))
    }
}

impl View for Wire {
    fn element(&self) -> Option<&'static str> {
        Some("wire")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        // let mut path = Path::new();

        // let mut paint = Paint::color(Color::black());
        // paint.set_line_width(1.0);

        // path.move_to(bounds.left(), bounds.top());
        // path.line_to(bounds.right(), bounds.top());
        // path.line_to(bounds.right(), bounds.bottom());
        // path.line_to(bounds.left(), bounds.bottom());
        // path.line_to(bounds.left(), bounds.top());

        // canvas.stroke_path(&mut path, &paint);
        // path.move_to(self.from.0, self.from.1);
        // path.line_to(self.to.0, self.to.1);

        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 1.0));
        // paint.set_line_width(1.0);
        let mut path = Path::new();
        path.move_to(20.0, 100.0);
        path.line_to(40.0, 100.0);

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
