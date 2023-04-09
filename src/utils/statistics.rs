pub fn mean(data: &[f64]) -> f64 {
    let sum = data.iter().sum::<f64>();
    let count = data.len() as f64;

    sum / count
}

pub fn std_deviation(data: &[f64]) -> f64 {
    let variance = data
        .iter()
        .map(|value| {
            let diff = mean(data) - *value;

            diff * diff
        })
        .sum::<f64>()
        / data.len() as f64;

    variance.sqrt()
}
