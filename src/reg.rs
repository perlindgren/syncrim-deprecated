use crate::port::{Port, PORT_SIZE};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Lens, Clone)]
pub struct Reg {
    state: u32,        // current state
    history: Vec<u32>, // history buffer
}

#[derive(Debug)]
pub enum RegEvent {
    Clk,
    SetTime(usize), // set to time
}

impl Model for Reg {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            RegEvent::Clk => {
                println!("reg clk");
                // not yet sure where it should get the data from ...
            }
            RegEvent::SetTime(cycle) => {
                self.state = *self.history.get(*cycle).unwrap();
            }
        });
    }
}

impl Reg {
    pub fn new<'a>(cx: &'a mut Context, reset: u32, x: f32, y: f32) -> Handle<'a, Self> {
        let half_width = PORT_SIZE;
        let half_hight = PORT_SIZE * 0.5;

        vizia::prelude::View::build(
            Self {
                state: reset,
                history: vec![reset],
            },
            cx,
            |cx| {
                // input
                Port::new(cx, half_width * 0.5, half_hight);
                // output
                Port::new(cx, half_width * 1.5, half_hight);
            },
        )
        .position_type(PositionType::SelfDirected)
        .left(Pixels(x - half_width))
        .top(Pixels(y - half_hight))
        .width(Pixels(half_width * 2.0))
        .height(Pixels(half_hight * 2.0))
    }
}

impl View for Reg {
    fn element(&self) -> Option<&'static str> {
        Some("Reg")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("Register draw {:?}", bounds);
        println!("-- for now just printing state -- {}", self.state);

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
