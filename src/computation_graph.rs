pub type Data = Vec<u8>;
pub type ConnDescriptor = Vec<Vec<usize>>; // B_inp -> A_outs

pub trait Computer {
    fn set_input_vals(&mut self, data: &Data, conn_descriptor: &ConnDescriptor);
    fn compute(&self) -> Data;
}

pub struct Graph<'a> {
    adj_mtrx:               Vec<Vec<bool>>,
    nodes:                  Vec<Box<dyn Computer>>,
    node_conn_descriptors:  Vec<ConnDescriptor>,

    node_vals:              Vec<Option<&'a Data>>,
}

impl Graph {
    pub fn new(
        adj_mtrx: Vec<Vec<bool>>,
        nodes: Vec<Box<dyn Computer>>,
        node_conn_descriptors: Vec<ConnDescriptor>,
    ) -> Self {
        if adj_mtrx.len() != nodes.len() {
            panic!("adj_mtrx and nodes lens mismatch")
        }

        if adj_mtrx.len() == 0 || nodes.len() == 0 {
            panic!("zero size not allowed")
        }

        let node_vals =
            vec![Option::None; adj_mtrx.len()];

        Self {
            adj_mtrx,
            nodes,
            node_conn_descriptors,
            node_vals
        }
    }

    fn find_terminal_nodes(&self, input_nodes: bool) -> Vec<usize> {
        let mut nodes: Vec<usize> = Vec::new();

 'nodes:for i in 0..self.nodes.len() {
            for j in 0..self.nodes.len() {
                let connection = match input_nodes {
                    true => self.adj_mtrx[j][i],
                    false => self.adj_mtrx[i][j],
                };
                if connection == true {
                    continue 'nodes
                }
            }
            nodes.push(i);
        }
        nodes
    }

    fn find_predecessors(&self, node: usize) -> Vec<usize> {
        let mut nodes: Vec<usize> = Vec::new();

        for i in 0..self.nodes.len() {
            if self.adj_mtrx[i][node] == true {
                nodes.push(i);
            }
        }
        nodes
    }

    fn compute_node(&self, node: usize) {
        if self.node_vals[node] != None {
            return;
        }

        let predecessors = self.find_predecessors(node);
        for i in predecessors {
            if self.node_vals[i] == None {
                self.compute_node(i);
            }
            let predecessor_data = self.node_vals[i].unwrap();
            let conn_descriptor = &self.node_conn_descriptors[i];
            self.nodes[i].set_input_vals(predecessor_data, conn_descriptor);
        }
        self.node_vals[node] = Some(&self.nodes[node].compute());
    }
}

impl Computer for Graph {
    fn set_input_vals(&mut self, data: &Data, conn_descriptor: &ConnDescriptor) {
        let input_nodes= self.find_terminal_nodes(true);

        for i in input_nodes {
            let idxes = &conn_descriptor[i];
            let view: Data = idxes.iter().map(|&i| &data[i]).collect();
            self.node_vals[i] = Some(&view);
        }
    }

    fn compute(&self) -> Data {
        let output_nodes = self.find_terminal_nodes(false);
        for o in output_nodes {
            self.compute_node(o);
        }
        let mut output = Data::new();
        let output_nodes = self.find_terminal_nodes(false);
        for o in output_nodes {
            output.append(&mut self.node_vals[o].unwrap());
        }
        output
    }
}

pub mod fundamental_computers {
    use crate::computation_graph::{Computer, ConnDescriptor, Data};

    pub struct Nand<'a> {
        inp1: Option<&'a Data>,
        inp2: Option<&'a Data>,
    }

    impl Nand {
        fn new() -> Self {
            Self {
                inp1: None,
                inp2: None,
            }
        }
    }

    impl Computer for Nand {
        fn set_input_vals(&mut self, data: &Data, conn_descriptor: &ConnDescriptor) {
            if conn_descriptor.len() != 2 {
                panic!("specified incorrect number of target inputs")
            }
            if conn_descriptor[0].len() != conn_descriptor[1].len() {
                panic!("input sizes mismatch")
            }

            let view1: &Data = conn_descriptor[0].iter().map(
                |&i| &data[i]
            ).collect();
            let view2: &Data = conn_descriptor[1].iter().map(
                |&i| &data[i]
            ).collect();

            self.inp1 = Some(view1);
            self.inp2 = Some(view2);
        }

        fn compute(&self) -> Data {
            let outp = Data::with_capacity(self.inp1.len());
            for i in 0..outp.len() {
                outp[i] = !(self.inp1[i] && self.inp2[i]);
            }
            outp
        }
    }
}