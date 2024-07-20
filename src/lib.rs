mod computation_graph;
mod common_computers;

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

    mod nand_tests {
        use crate::common_computers::NAND;
        use crate::computation_graph::Computer;

        fn create_nand() -> Box<dyn Computer> {
            Box::new(NAND::new())
        }

        #[test]
        fn test_nands_unit_bit() {
            let mut nand = create_nand();

            let inp = vec![1, 1];
            assert_eq!(vec![0], nand.compute(&inp));

            let inp = vec![1, 0];
            assert_eq!(vec![1], nand.compute(&inp));

            let inp = vec![0, 1];
            assert_eq!(vec![1], nand.compute(&inp));

            let inp = vec![0, 0];
            assert_eq!(vec![1], nand.compute(&inp));

            let inp = vec![0, 1,
                                    1, 0];
            assert_eq!(vec![1, 1], nand.compute(&inp));
        }
    }

    mod not_tests {
        use crate::common_computers::NOT;
        use crate::computation_graph::{Computer, Data};

        fn create_not() -> Box<dyn Computer> {
            Box::new(NOT::new())
        }

        #[test]
        fn test_not() {
            let mut not = create_not();

            let inp: Data = vec![0];
            assert_eq!(not.compute(&inp), vec![1]);

            let inp: Data = vec![1];
            assert_eq!(not.compute(&inp), vec![0]);
        }
    }
}
