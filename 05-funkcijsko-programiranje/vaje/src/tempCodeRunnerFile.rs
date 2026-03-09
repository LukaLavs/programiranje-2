fn sum_pairs(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    v2.iter().zip(v1.iter()).map(|(x, y)| x + y).collect()
}