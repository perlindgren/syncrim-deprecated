use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
enum ComponentType {
    Register,
    Mux,
    Wire,
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    id: String,
    index: usize,
}

impl Input {
    fn new(id: String, index: usize) -> Self {
        Self { id, index }
    }
}

#[derive(Debug, Serialize, Deserialize)]
// for now u32
struct Value(u32);

#[derive(Debug, Serialize, Deserialize)]
struct Output(Value);

type Inputs = Vec<Input>;
type Outputs = Vec<Output>;

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Size {
    width: f32,
    height: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComponentStore {
    id: String,
    component_type: ComponentType,
    pos: Position,
    opt_size: Option<Size>,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

// impl Component {
//     fn new(inputs: Inputs, outputs: Outputs, component_type: ComponentType) -> Self {
//         Self {
//             inputs,
//             outputs,
//             component_type,
//         }
//     }
// }

// trait Propagate {
//     fn propagate(&mut self);
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Mux {}

// impl Propagate for Component<Mux> {
//     fn propagate(&mut self) {
//         println!("propagate")
//     }
// }

fn main() {
    // let mut hm = HashMap::new();
    let mux_1 = ComponentStore {
        id: "mux_1".to_string(),
        component_type: ComponentType::Mux,
        pos: Position { x: 100.0, y: 100.0 },
        opt_size: None,
        inputs: vec![],
        outputs: vec![],
    };

    let reg_1 = ComponentStore {
        id: "reg_1".to_string(),
        component_type: ComponentType::Mux,
        pos: Position { x: 80.0, y: 100.0 },
        opt_size: None,
        inputs: vec![],
        outputs: vec![],
    };

    let components = vec![mux_1, reg_1];

    let serialized = serde_json::to_string(&components).unwrap();

    println!("serialized = {}", serialized);

    // let mux2: Component<Mux> = serde_json::from_str(&serialized).unwrap();

    // println!("mux deser {:?}", mux2);
    // // assert!(hm.get("a") == Some(&0))
}
