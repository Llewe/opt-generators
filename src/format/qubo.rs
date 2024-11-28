use rand::prelude::StdRng;
use crate::format::Format;
use serde::{Deserialize, Serialize};
use rand::{Rng, SeedableRng};
use pyo3::{pyclass, pymethods, PyResult};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[pyclass]
pub struct Qubo {
    matrix: Vec<Vec<f64>>,
}

impl Format for Qubo {
    fn to_string(&self) -> String {
        serde_json::to_string::<Qubo>(&self).unwrap()
    }

    fn from_json() -> Self {
        todo!()
    }

    fn random(size: usize, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);

        let mut matrix: Vec<Vec<f64>> = Vec::with_capacity(size);


        for i in 0..size {
            let mut row: Vec<f64> = Vec::with_capacity(size);
            for j in 0..size {
                if i <= j {
                    row.push((rng.gen::<f64>() * 1000.0).round() / 1000.0);
                } else {
                    row.push(matrix[j][i]);
                }
            }
            matrix.push(row);
        }


        Qubo {
            matrix
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let qubo = Qubo {
            matrix: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        };
        let expected_json = "{\"matrix\":[[1.0,2.0],[3.0,4.0]]}";
        assert_eq!(qubo.to_string(), expected_json);
    }

    #[test]
    fn test_seed() {
        let qubo1 = Qubo::random(10, 42);
        let qubo2 = Qubo::random(10, 42);

        assert_eq!(qubo1, qubo2);
    }

    #[test]
    fn test_matrix_symmetry() {
        let size = 10;
        let qubo = Qubo::random(size, 42);

        for i in 0..size {
            for j in 0..size {
                // Assert that matrix[i][j] == matrix[j][i]
                assert_eq!(qubo.matrix[i][j], qubo.matrix[j][i], "Matrix is not symmetric at position ({}, {})", i, j);
            }
        }
    }
}