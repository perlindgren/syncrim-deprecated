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
    name: String,
    index: usize,
}

impl Input {
    fn new(name: String, index: usize) -> Self {
        Self { name, index }
    }
}
// for now u32
#[derive(Debug, Serialize, Deserialize)]
struct Output(u32);

type Inputs = Vec<Input>;
type Outputs = Vec<Output>;

#[derive(Debug, Serialize, Deserialize)]
struct Component {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    component_type: ComponentType,
}

impl Component {
    fn new(inputs: Inputs, outputs: Outputs, component_type: ComponentType) -> Self {
        Self {
            inputs,
            outputs,
            component_type,
        }
    }
}

trait Propagate {
    fn propagate(&mut self);
}

#[derive(Debug, Serialize, Deserialize)]
struct Mux {}

impl Propagate for Component<Mux> {
    fn propagate(&mut self) {
        println!("propagate")
    }
}

fn main() {
    // let mut hm = HashMap::new();
    let mux = Component::new(vec![Input::new("pc_reg".to_owned(), 0)], vec![], Mux {});

    println!("mux {:?}", mux);
    let serialized = serde_json::to_string(&mux).unwrap();

    println!("serialized = {}", serialized);

    let mux2: Component<Mux> = serde_json::from_str(&serialized).unwrap();

    println!("mux deser {:?}", mux2);
    // assert!(hm.get("a") == Some(&0))
}
