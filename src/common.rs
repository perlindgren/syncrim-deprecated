// const PORT_SIZE: f32 = 10.0;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum ComponentType {
    Register,
    Mux,
    Wire,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Size {
    width: f32,
    height: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    pub id: String,
    pub index: usize,
}

pub struct Ports {
    pub inputs: Vec<usize>,
    pub outputs_start_index: usize,
}
