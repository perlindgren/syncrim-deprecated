use crate::common::*;
use std::convert::TryFrom;

struct ConstModel {}

impl Eval for ConstModel {
    fn clk(&mut self, curr_state: &SimState, sim_state: &mut SimState) {
        println!("Eval:ConstModel");
    }
}

impl TryFrom<(&ComponentStore, &IdIndex)> for ConstModel {
    type Error = ();

    fn try_from(store_id_index: (&ComponentStore, &IdIndex)) -> Result<Self, Self::Error> {
        let (c, _i) = store_id_index;
        if c.component_type == ComponentType::Constant {
            Ok(ConstModel {})
        } else {
            Err(())
        }
    }
}

struct AddModel {
    in_a: usize,
    in_b: usize,
    output: usize,
}

impl Eval for AddModel {
    fn clk(&mut self, curr_state: &SimState, sim_state: &mut SimState) {
        println!("Eval:AddModel");
        let a = curr_state.get(self.in_a);
        let b = curr_state.get(self.in_b);

        sim_state.set(self.output, a + b);
    }
}

impl TryFrom<(&ComponentStore, &IdIndex)> for AddModel {
    type Error = ();

    fn try_from(store_id_index: (&ComponentStore, &IdIndex)) -> Result<Self, Self::Error> {
        let (c, i) = store_id_index;
        if c.component_type == ComponentType::Adder {
            let in_a = i.get_in(c.inputs.get(0).unwrap());
            let in_b = i.get_in(c.inputs.get(1).unwrap());
            let output = i.get_out_start(&c.id);
            Ok(AddModel { in_a, in_b, output })
        } else {
            Err(())
        }
    }
}

struct MuxModel {
    select: usize,
    inputs: Vec<usize>,
    output: usize,
}

impl Eval for MuxModel {
    fn clk(&mut self, curr_state: &SimState, sim_state: &mut SimState) {
        let select = curr_state.get(self.select);
        let selected = curr_state.get(*self.inputs.get(select as usize).unwrap());
        sim_state.set(self.output, selected);
    }
}

impl TryFrom<(&ComponentStore, &IdIndex)> for MuxModel {
    type Error = ();

    fn try_from(store_id_index: (&ComponentStore, &IdIndex)) -> Result<Self, Self::Error> {
        let (c, i) = store_id_index;
        if c.component_type == ComponentType::Register {
            let select = i.get_in(c.inputs.get(0).unwrap());
            let in_slice = &c.inputs.as_slice()[1..];
            let inputs = i.get_ins(in_slice);
            let output = i.get_out_start(&c.id);

            Ok(MuxModel {
                select,
                inputs,
                output,
            })
        } else {
            Err(())
        }
    }
}

pub struct RegModel {
    pub id: String,
    pub input: usize,
    pub output: usize,
}

impl Eval for RegModel {
    fn clk(&mut self, curr_state: &SimState, next_state: &mut SimState) {
        println!("reg {} clk {} {}", self.id, self.input, self.output);
        let input = curr_state.get(self.input);
        println!("in val {}", input);
        next_state.set(self.output, input);
    }
}

impl TryFrom<(&ComponentStore, &IdIndex)> for RegModel {
    type Error = ();

    fn try_from(store_id_index: (&ComponentStore, &IdIndex)) -> Result<Self, Self::Error> {
        let (c, i) = store_id_index;
        if c.component_type == ComponentType::Register {
            Ok(RegModel {
                id: c.id.clone(),
                input: i.get_in(c.inputs.get(0).unwrap()),
                output: i.get_out_start(&c.id),
            })
        } else {
            Err(())
        }
    }
}

pub struct Evaluator {
    pub evaluator: Box<dyn Eval>,
}

impl TryFrom<(&ComponentStore, &IdIndex)> for Evaluator {
    type Error = ();
    fn try_from(store_id_index: (&ComponentStore, &IdIndex)) -> Result<Self, Self::Error> {
        let evaluator: Box<dyn Eval> = if let Ok(m) = RegModel::try_from(store_id_index) {
            Box::new(m)
        } else if let Ok(m) = ConstModel::try_from(store_id_index) {
            Box::new(m)
        } else if let Ok(m) = AddModel::try_from(store_id_index) {
            Box::new(m)
        } else if let Ok(m) = MuxModel::try_from(store_id_index) {
            Box::new(m)
        } else {
            Err(())?
        };
        Ok(Evaluator { evaluator })
    }
}

pub struct SimModel {
    pub evaluators: Vec<Evaluator>,
}

impl Eval for SimModel {
    fn clk(&mut self, curr_state: &SimState, next_state: &mut SimState) {
        self.evaluators
            .iter_mut()
            .for_each(|e| e.evaluator.clk(curr_state, next_state))
    }
}

impl TryFrom<(Vec<ComponentStore>, &IdIndex)> for SimModel {
    type Error = ();

    fn try_from(vcsi: (Vec<ComponentStore>, &IdIndex)) -> Result<Self, Self::Error> {
        let (vcs, id_index) = vcsi;

        let evaluators = vcs
            .iter()
            .map(|cs| Evaluator::try_from((cs, id_index)).expect("invalid component"))
            .collect();

        Ok(SimModel { evaluators })
    }
}
