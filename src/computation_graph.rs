pub type Data = Vec<u8>;

pub trait Computer {
    fn input_pins_num(&self) -> usize;
    fn compute(&mut self, data: &Data) -> Data;
}

pub struct Graph {
    adj_mtrx:               Vec<Vec<bool>>,
    nodes:                  Vec<Box<dyn Computer>>,
    node_vals:              Vec<Option<Data>>,
}

impl Graph {
    pub fn new(
        adj_mtrx: Vec<Vec<bool>>,
        nodes: Vec<Box<dyn Computer>>,
    ) -> Self {
        if adj_mtrx.len() != nodes.len() {
            panic!("adj_mtrx and nodes lens mismatch")
        }

        if adj_mtrx.len() == 0 || nodes.len() == 0 {
            panic!("zero size not allowed")
        }

        let node_vals = vec![None; nodes.len()];

        Self {
            adj_mtrx,
            nodes,
            node_vals,
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

    fn compute_initial_nodes(&mut self, data: &Data) {
        let initial_nodes = self.find_terminal_nodes(true);

        let mut current_start: usize = 0;
        for i in initial_nodes {
            let pins_num = self.nodes[i].input_pins_num();
            let current_end = current_start + pins_num;
            let input_data = data[current_start..current_end].to_vec();
            current_start = current_end;
            let input_activation = self.nodes[i].compute(&input_data);
            let _ = self.node_vals[i].insert(input_activation);
        }
    }

    fn compute_node(&mut self, node: usize) {
        if self.node_vals[node] != None {
            return;
        }

        let predecessors = self.find_predecessors(node);

        if predecessors.len() == 0 {
            panic!("uninitialized graph")
        }

        let mut predecessors_data = Data::new();

        for &i in &predecessors {
            if self.node_vals[i] == None {
                self.compute_node(i);
            }
        }

        for &i in &predecessors {
            let data = self.node_vals[i].as_ref().unwrap();
            predecessors_data.append(&mut Vec::clone(data));
        }
        let node_val = self.nodes[node].compute(&predecessors_data);
        let _ = self.node_vals[node].insert(node_val);
    }
}

impl Computer for Graph {
    fn input_pins_num(&self) -> usize {
        let mut count: usize = 0;
        let input_nodes = self.find_terminal_nodes(true);

        for i in input_nodes {
            count += self.nodes[i].input_pins_num();
        }
        count
    }

    fn compute(&mut self, data: &Data) -> Data {
        self.compute_initial_nodes(data);

        let output_nodes = self.find_terminal_nodes(false);
        for &o in &output_nodes {
            self.compute_node(o);
        }

        let mut output = Data::new();
        for &o in &output_nodes {
            output.append(&mut Vec::clone(self.node_vals[o].as_ref().unwrap()));
        }
        output
    }
}
