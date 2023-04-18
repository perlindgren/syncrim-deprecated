use syncrim::{reg_view::RegView, wire::Wire};

use vizia::prelude::*;

#[derive(Debug)]
pub enum SimEvent {
    Clk,
    SetCycle(usize),
}

#[derive(Lens, Debug, Clone)]
pub struct RegModel {
    state: u32,
    history: Vec<u32>,
}

use std::fmt;

impl fmt::Display for RegModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

impl Data for RegModel {
    fn same(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl RegModel {
    fn new(reset: u32) -> Self {
        Self {
            state: reset,
            history: vec![reset],
        }
    }
}

impl Model for RegModel {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            SimEvent::Clk => {
                self.state = self.state + 1;
            }
            SimEvent::SetCycle(cycle) => {
                todo!()
            }
        });
    }
}

#[derive(Lens, Debug)]
pub struct SimState {
    pc_reg: RegModel,
}

impl Model for SimState {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.pc_reg.event(cx, event);
    }
}

fn main() {
    Application::new(|cx| {
        SimState {
            pc_reg: RegModel::new(42),
        }
        .build(cx);

        Button::new(cx, |cx| cx.emit(SimEvent::Clk), |cx| Label::new(cx, "Clk"));

        let _sync = RegView::new(cx, SimState::pc_reg, 200.0, 200.0);
        // let _w1 = Wire::new(cx, 205.0, 200.0, 230.0, 200.0);
        // let _w1 = Wire::new(cx, 230.0, 200.0, 230.0, 220.0);
        // let _w1 = Wire::new(cx, 230.0, 220.0, 170.0, 220.0);
        // let _w1 = Wire::new(cx, 170.0, 220.0, 170.0, 200.0);
        // let _w1 = Wire::new(cx, 170.0, 200.0, 195.0, 200.0);
    })
    .run();
}
