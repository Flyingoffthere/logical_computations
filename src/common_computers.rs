use super::computation_graph::{Computer, Data};

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