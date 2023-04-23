// use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// use syncrim::models::RegModel;
// use syncrim::reg_view::RegView;
// use vizia::prelude::*;

use syncrim::common::*;
use syncrim::models::*;

fn main() {
    let const_1 = ComponentStore {
        id: "const_1".to_string(),
        component_type: ComponentType::Constant,
        pos: Position { x: 100.0, y: 100.0 },
        opt_size: None,
        inputs: vec![],
        outputs: vec![Output::Constant(4)],
    };

    let add_1 = ComponentStore {
        id: "add_1".to_string(),
        component_type: ComponentType::Adder,
        pos: Position { x: 60.0, y: 100.0 },
        opt_size: None,
        inputs: vec![
            Input {
                id: "add_1".to_string(),
                index: 0,
            },
            Input {
                id: "const_1".to_string(),
                index: 0,
            },
        ],
        outputs: vec![Output::Synchronous],
    };

    let components = vec![const_1, add_1];

    let serialized = serde_json::to_string(&components).unwrap();

    println!("serialized = {}", serialized);

    let mut store = HashMap::new();
    for c in components.clone() {
        store.insert(c.id.clone(), c);
    }

    let mut sim_state = SimState { values: vec![] };
    let mut id_index = IdIndex(HashMap::new());
    let mut id_id = IdId(HashMap::new());

    // allocate storage for lensed outputs
    for c in components.clone() {
        // start index for outputs related to component
        id_index
            .0
            .insert(c.id.clone(), sim_state.values.len().clone());

        // build topological dependency
        for input in c.inputs {
            id_id.0.insert(c.id.clone(), input);
        }
        for output in c.outputs {
            sim_state.values.push(match output {
                Output::Constant(v) => v,
                _ => 0,
            });
        }
    }

    println!("id_index {:?}", id_index);
    println!("id_id {:?}", id_id);

    println!("sim_state {:?}", sim_state);

    let mut sm = SimModel::try_from((components.clone(), &id_index)).unwrap();

    println!("SimModel {:?}", sm.evaluators.len());
    let curr_state = sim_state.clone();
    let mut next_state = sim_state;

    sm.clk(&curr_state, &mut next_state);

    println!("curr_state {:?}", curr_state);
    println!("next_state {:?}", next_state);

    // let curr_state = next_state.clone();

    // sm.clk(&curr_state, &mut next_state);

    // println!("curr_state {:?}", curr_state);
    // println!("next_state {:?}", next_state);

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
