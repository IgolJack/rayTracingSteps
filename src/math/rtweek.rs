use rand::Rng;

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {return min}
    if x > max {return max}
    return x
}