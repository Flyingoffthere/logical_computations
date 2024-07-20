use super::computation_graph::{Computer, Data, Graph};

pub struct NAND{}

impl NAND {
    pub fn new() -> Self {
        Self{}
    }
}

impl Computer for NAND {
    fn input_pins_num(&self) -> usize {
        2
    }

    fn compute(&mut self, data: &Data) -> Data {
        if data.len() % 2 != 0 {
            panic!("incorrect data")
        }

        let mut out = Data::new();
        let boundary = data.len() / 2;
        let inp1 = data[0..boundary].to_vec();
        let inp2 = data[boundary..data.len()].to_vec();

        for i in 0..boundary {
            out.push(!(inp1[i] & inp2[i]) & 1);
        }

        out
     }
}

pub struct NOT {
    backbone: Graph,
}

impl NOT {
    pub fn new() -> Self {
        let adj_mtrx = vec![vec![false]];
        let nand: Box<dyn Computer> = Box::new(NAND::new());
        let nodes = vec![nand];
        let g = Graph::new(adj_mtrx, nodes);

        Self {
            backbone: g
        }
    }
}

impl Computer for NOT {
    fn input_pins_num(&self) -> usize {
        1
    }

    fn compute(&mut self, data: &Data) -> Data {
        let mut inp = data.clone();
        let mut inp_cloned = data.clone();
        inp.append(&mut inp_cloned);
        self.backbone.compute(&inp)
    }
}