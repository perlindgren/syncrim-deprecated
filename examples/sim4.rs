use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;
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

// #[derive(Lens, Debug, Clone)]
// struct Comp<T: Clone + 'static> {
//     output: Vec<u32>,
//     _phantom: PhantomData<T>,
// }

#[derive(Lens, Debug)]
pub struct SimState {
    values: Vec<u32>,
}

impl Model for SimState {}

use syncrim::reg_view::RegView;

// struct Component<T> {
//     ports: Ports,
//     _phantom: PhantomData<T>,
// }

struct Mux {
    ports: Ports,
}

trait Eval {
    fn eval(&mut self, sim_state: &mut SimState);
}

impl Eval for Mux {
    fn eval(&mut self, sim_state: &mut SimState) {}
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
        inputs: vec![],
        outputs: 2,
    };

    let components = vec![mux_1, reg_1];

    let serialized = serde_json::to_string(&components).unwrap();

    println!("serialized = {}", serialized);

    let mut store = HashMap::new();
    for c in components.clone() {
        store.insert(c.id.clone(), c);
    }

    // let mux2: Component<Mux> = serde_json::from_str(&serialized).unwrap();

    // println!("mux deser {:?}", mux2);
    // // assert!(hm.get("a") == Some(&0))

    let mut sim_state = SimState { values: vec![] };
    let mut hm = HashMap::new();

    // allocate storage for lensed outputs
    for c in components.clone() {
        // start index for outputs related to component
        hm.insert(c.id.clone(), sim_state.values.len().clone());
        for _ in 0..c.outputs {
            sim_state.values.push(0);
        }
    }

    // generate evaluators
    // let mut sim_eval = vec![];
    for c in components.clone() {
        // start index for outputs related to component
        let id = c.id;
        let outputs_start_index = *hm.get(&id).unwrap();

        // hm.get(c.id);
        let mut inputs = vec![];
        for input in c.inputs {
            let r = hm.get(&input.id).unwrap();
            inputs.push(*r);
        }

        let ports = Ports {
            inputs,
            outputs_start_index,
        };
    }

    println!("sim_state {:?}", sim_state);
    println!("hm {:?}", hm);

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
