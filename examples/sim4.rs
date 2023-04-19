use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use syncrim::reg_view::RegView;
use vizia::prelude::*;

use syncrim::common::*;

#[derive(Debug, Serialize, Deserialize)]
// for now u32
struct Value(u32);

#[derive(Debug, Serialize, Deserialize)]
struct Output(Value);

type Inputs = Vec<Input>;
type Outputs = Vec<Output>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ComponentStore {
    id: String,
    component_type: ComponentType,
    pos: Position,
    opt_size: Option<Size>,
    inputs: Vec<Input>,
    outputs: usize,
}

trait Eval {
    fn clk(&mut self, sim_state: &mut SimState);
}

struct MuxModel {
    select: usize,
    inputs: Vec<usize>,
    output: usize,
}

impl Eval for MuxModel {
    fn clk(&mut self, sim_state: &mut SimState) {
        let select = sim_state.get(self.select);
        let selected = sim_state.get(*self.inputs.get(select as usize).unwrap());
        sim_state.set(self.output, selected);
    }
}

struct RegModel {
    input: usize,
    output: usize,
}

impl Eval for RegModel {
    fn clk(&mut self, sim_state: &mut SimState) {
        let input = sim_state.get(self.input);
        sim_state.set(self.output, input);
    }
}

fn main() {
    let mux_1 = ComponentStore {
        id: "mux_1".to_string(),
        component_type: ComponentType::Mux,
        pos: Position { x: 100.0, y: 100.0 },
        opt_size: None,
        inputs: vec![],
        outputs: 1,
    };

    let reg_1 = ComponentStore {
        id: "reg_1".to_string(),
        component_type: ComponentType::Register,
        pos: Position { x: 80.0, y: 100.0 },
        opt_size: None,
        inputs: vec![Input {
            id: "mux_1".to_string(),
            index: 0,
        }],
        outputs: 1,
    };

    let components = vec![mux_1, reg_1];

    let serialized = serde_json::to_string(&components).unwrap();

    println!("serialized = {}", serialized);

    let mut store = HashMap::new();
    for c in components.clone() {
        store.insert(c.id.clone(), c);
    }

    let mut sim_state = SimState { values: vec![] };
    let mut id_index = IdIndex(HashMap::new());

    // allocate storage for lensed outputs
    for c in components.clone() {
        // start index for outputs related to component
        id_index
            .0
            .insert(c.id.clone(), sim_state.values.len().clone());
        for _ in 0..c.outputs {
            sim_state.values.push(0);
        }
    }

    // generate evaluators
    // let mut sim_eval = vec![];

    let mut sim = vec![];
    for c in components.clone() {
        // start index for outputs related to component
        let outputs_start_index = id_index.get_out_start(&c.id);

        match c.component_type {
            ComponentType::Register => {
                let r = RegModel {
                    input: id_index.get_in(c.inputs.get(0).unwrap()),
                    output: outputs_start_index,
                };
                sim.push(r);
            }
            ComponentType::Mux => {}
            ComponentType::Wire => {}
        }
    }

    println!("sim_state {:?}", sim_state);

    sim_state.set(0, 42);
    println!("sim_state {:?}", sim_state);

    for e in &mut sim {
        e.clk(&mut sim_state);
    }

    println!("sim_state {:?}", sim_state);

    // allocate value space

    // Application::new(|cx| {
    //     sim_state.build(cx);

    //     for (id, comp) in store {
    //         match comp.component_type {
    //             ComponentType::Mux => {}
    //             ComponentType::Register => {
    //                 println!("here");
    //                 RegView::new(cx, SimState::x, comp.pos);
    //             }
    //             ComponentType::Wire => {}
    //         }
    //     }

    //     // let _sync = Register::new(cx, 200.0, 200.0);
    //     // let _w1 = Wire::new(cx, 205.0, 200.0, 230.0, 200.0);
    //     // let _w1 = Wire::new(cx, 230.0, 200.0, 230.0, 220.0);
    //     // let _w1 = Wire::new(cx, 230.0, 220.0, 170.0, 220.0);
    //     // let _w1 = Wire::new(cx, 170.0, 220.0, 170.0, 200.0);
    //     // let _w1 = Wire::new(cx, 170.0, 200.0, 195.0, 200.0);
    // })
    // .run();
}
