// const PORT_SIZE: f32 = 10.0;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vizia::prelude::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum ComponentType {
    Register,
    Constant,
    Adder,
    Mux,
    Wire,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Output {
    Combinatorial,
    Synchronous,
    Constant(u32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentStore {
    pub id: String,
    pub component_type: ComponentType,
    pub pos: Position,
    pub opt_size: Option<Size>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
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

#[derive(Debug)]
pub struct IdIndex(pub HashMap<String, usize>);

#[derive(Debug)]
pub struct IdId(pub HashMap<String, Input>);

impl IdIndex {
    pub fn get_in(&self, input: &Input) -> usize {
        *self.0.get(&input.id).unwrap() + input.index
    }

    pub fn get_ins(&self, inputs: &[Input]) -> Vec<usize> {
        inputs.iter().map(|i| self.get_in(i)).collect()
    }

    pub fn get_out_start(&self, id: &String) -> usize {
        *self.0.get(id).unwrap()
    }
}

#[derive(Lens, Debug, Clone)]
pub struct SimState {
    pub values: Vec<u32>,
}

impl SimState {
    pub fn get(&self, index: usize) -> u32 {
        *self.values.get(index).unwrap()
    }

    pub fn set(&mut self, index: usize, value: u32) {
        let val_ref = self.values.get_mut(index).unwrap();
        *val_ref = value
    }
}

impl Model for SimState {}

pub trait Eval {
    fn clk(&mut self, curr_state: &SimState, next_state: &mut SimState);
}
