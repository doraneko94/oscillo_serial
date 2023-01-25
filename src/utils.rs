use ndarray::*;

pub fn min_max(data: &Array2<f32>, count: usize) -> (f32, f32) {
    let (mut min, mut max) = (std::f32::INFINITY, std::f32::NEG_INFINITY);
    let shape = data.shape();
    for yi in 0..shape[0] {
        for xi in 0..std::cmp::min(shape[1], count) {
            if min > data[[yi, xi]] { min = data[[yi, xi]]; }
            if max < data[[yi, xi]] { max = data[[yi, xi]]; }
        }
    }
    (min, max)
}