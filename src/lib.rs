mod computation_graph;

#[cfg(test)]
mod tests {
    mod graph_mock_tests {
        use crate::computation_graph::{Graph, Computer, Data};

        fn create_mock_computer() -> Box<dyn Computer> {
            struct Mock{}

            impl Computer for Mock {
                fn input_pins_num(&self) -> usize {
                    2
                }

                fn compute(&mut self, data: &Data) -> Data {
                    vec![1]
                }
            }
            Box::new(Mock{})
        }

        #[test]
        fn test_mocks() {
            let mock1 = create_mock_computer();
            let mock2 = create_mock_computer();
            let mock3 = create_mock_computer();

            let adj_mtrx = vec![
                vec![false, false, true],
                vec![false, false, true],
                vec![false, false, false],
            ];
            let nodes = vec![mock1, mock2, mock3];
            let mut g = Graph::new(adj_mtrx, nodes);

            let inps = vec![1, 1, 0, 1];

            assert_eq!(g.input_pins_num(), 4);
            assert_eq!(g.compute(&inps), vec![1]);
        }
    }
}
