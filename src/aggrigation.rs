
pub fn sum(values: Vec<f64>) -> f64 {
    let mut total = 0.0;
    for value in values {
        total += value;
    }   
    total
}   



pub fn avg(values: Vec<f64>) -> f64 {
    let mut total = 0.0;
    for value in &values {
        total += value;
    }   
    total / values.len() as f64
}   




pub fn product(values: Vec<f64>) -> f64 {
    let mut total = 1.0;
    for value in values {
        total *= value;
    }   
    total
}   



pub fn min(values: Vec<f64>) -> f64 {
    let mut min = f64::MAX;
    for value in values {
        if value < min {
            min = value;
        }
    }   
    min
}   



pub fn max(values: Vec<f64>) -> f64 {
    let mut max = f64::MIN;
    for value in values {
        if value > max {
            max = value;
        }
    }   
    max
}   