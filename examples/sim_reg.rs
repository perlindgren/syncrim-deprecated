use syncrim::{reg::Reg, wire::Wire};

use vizia::prelude::*;

#[derive(Lens, Debug)]
pub struct SimState {
    pc_reg: u32,
}

#[derive(Debug)]
pub enum SimEvent {
    GetPc,
    SetPc(u32),
}

impl Model for SimState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            SimEvent::SetPc(num) => {
                self.pc_reg = *num;
            }
            SimEvent::GetPc => {
                // hmm not the way to do it
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        SimState { pc_reg: 0 }.build(cx);

        let _sync = Reg::new(cx, 42, 200.0, 200.0);
        let _w1 = Wire::new(cx, 205.0, 200.0, 230.0, 200.0);
        let _w1 = Wire::new(cx, 230.0, 200.0, 230.0, 220.0);
        let _w1 = Wire::new(cx, 230.0, 220.0, 170.0, 220.0);
        let _w1 = Wire::new(cx, 170.0, 220.0, 170.0, 200.0);
        let _w1 = Wire::new(cx, 170.0, 200.0, 195.0, 200.0);
    })
    .run();
}
