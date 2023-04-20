use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use syncrim::models::RegModel;
use syncrim::reg_view::RegView;
use vizia::prelude::*;

use syncrim::common::*;
use syncrim::models::*;

#[derive(Debug, Serialize, Deserialize)]
// for now u32
struct Value(u32);

#[derive(Debug, Serialize, Deserialize)]
struct Output(Value);

type Inputs = Vec<Input>;
type Outputs = Vec<Output>;

fn main() {
    // let mux_1 = ComponentStore {
    //     id: "mux_1".to_string(),
    //     component_type: ComponentType::Mux,
    //     pos: Position { x: 100.0, y: 100.0 },
    //     opt_size: None,
    //     inputs: vec![],
    //     outputs: 1,
    // };

    let reg_1 = ComponentStore {
        id: "reg_1".to_string(),
        component_type: ComponentType::Register,
        pos: Position { x: 80.0, y: 100.0 },
        opt_size: None,
        inputs: vec![Input {
            id: "reg_2".to_string(),
            index: 0,
        }],
        outputs: 1,
    };

    let reg_2 = ComponentStore {
        id: "reg_2".to_string(),
        component_type: ComponentType::Register,
        pos: Position { x: 60.0, y: 100.0 },
        opt_size: None,
        inputs: vec![Input {
            id: "reg_1".to_string(),
            index: 0,
        }],
        outputs: 1,
    };

    let components = vec![reg_1, reg_2];

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

    println!("sim_state {:?}", sim_state);

    sim_state.set(0, 42);
    sim_state.set(1, 43);

    println!("sim_state {:?}", sim_state);

    let mut sm = SimModel::try_from((components.clone(), &id_index)).unwrap();

    let curr_state = sim_state.clone();
    let mut next_state = sim_state;

    sm.clk(&curr_state, &mut next_state);

    println!("curr_state {:?}", curr_state);
    println!("next_state {:?}", next_state);

    let curr_state = next_state.clone();

    sm.clk(&curr_state, &mut next_state);

    println!("curr_state {:?}", curr_state);
    println!("next_state {:?}", next_state);

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
