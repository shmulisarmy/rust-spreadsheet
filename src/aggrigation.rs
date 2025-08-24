use std::{collections::HashMap, sync::LazyLock};




type Aggregation = fn(values: Vec<f64>) -> f64;



static SUM: Aggregation = |values| {
    let mut total = 0.0;
    for value in values {
        total += value;
    }   
    total
};


fn sum(values: Vec<f64>) -> f64 {
    let mut total = 0.0;
    for value in values {
        total += value;
    }   
    total
}
static AVG: Aggregation = |values| {
    let mut total = 0.0;
    for value in &values {
        total += value;
    }   
    total / values.len() as f64
};




static PRODUCT: Aggregation = |values| {
    let mut total = 1.0;
    for value in &values {
        total *= value;
    }   
    total
};   



static MIN: Aggregation = |values| {
    let mut min = f64::MAX;
    for value in values {
        if value < min {
            min = value;
        }
    }   
    min
};



static MAX: Aggregation = |values| {
    let mut max = f64::MIN;
    for value in values {
        if value > max {
            max = value;
        }
    }   
    max
};   


pub static AGGREGATORS: LazyLock<HashMap<&'static str, &'static Aggregation>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("sum", &SUM);
    m.insert("avg", &AVG);
    m.insert("product", &PRODUCT);
    m.insert("min", &MIN);
    m.insert("max", &MAX);
    m
});