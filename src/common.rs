// const PORT_SIZE: f32 = 10.0;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vizia::prelude::*;

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

pub struct IdIndex(pub HashMap<String, usize>);

impl IdIndex {
    pub fn get_in(&self, input: &Input) -> usize {
        *self.0.get(&input.id).unwrap() + input.index
    }

    pub fn get_ins(&self, inputs: Vec<Input>) -> Vec<usize> {
        inputs.iter().map(|i| self.get_in(i)).collect()
    }

    pub fn get_out_start(&self, id: &String) -> usize {
        *self.0.get(id).unwrap()
    }
}

#[derive(Lens, Debug)]
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
