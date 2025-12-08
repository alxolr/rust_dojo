use linfa::traits::{Fit, Predict};
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use ndarray::{Array1, Array2};

use crate::error::{Error, Result};
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        let records = parse_input(input)?;
        let num_samples = records.nrows();

        // Create dataset for unsupervised learning with unit targets
        let targets: Array1<()> = Array1::from_elem(num_samples, ());
        let dataset = DatasetBase::new(records.clone(), targets);

        let n_clusters = 2;

        // Configure and run K-means
        let model = KMeans::params(n_clusters)
            .n_runs(100)
            .fit(&dataset)
            .map_err(|e| Error::KMeans(e.to_string()))?;

        // Get cluster assignments
        let assignments = model.predict(&records);

        println!("Cluster assignments: {:?}", assignments);
        // We need to find the k for the k means algorithm
        // In order to achieve that we will try different k values
        // After we will use a silhouette method to identify the best fitting k

        Ok(0)
    }
    pub fn part_2(_input: &str) -> Result<u64> {
        Ok(0)
    }
}

fn parse_input(input: &str) -> Result<Array2<f64>> {
    let points: Vec<f64> = input
        .lines()
        .flat_map(|line| {
            line.trim()
                .split(",")
                .flat_map(|val| val.parse::<f64>())
                .collect::<Vec<_>>()
        })
        .collect();

    let num_points = points.len() / 3;

    let arr = Array2::from_shape_vec((num_points, 3), points)?;

    Ok(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMON_INPUT: &str = r#"162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689"#;

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(Solution::part_1(COMMON_INPUT)?, 40);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}
