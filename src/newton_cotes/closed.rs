use itertools::Itertools;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, ParallelBridge, ParallelIterator,
};

fn get_coefficients(n: usize) -> Vec<f64> {
    assert!(n >= 2);

    match n {
        2 => vec![0.5, 0.5],
        3 => vec![1. / 3., 4. / 3., 1. / 3.],
        4 => vec![0.375, 1.125, 1.125, 0.375],
        5 => vec![14. / 45., 64. / 45., 8. / 15., 64. / 45., 14. / 45.],
        _ => unimplemented!(),
    }
}

fn _closed_newton_cotes(a: f64, b: f64, f: impl Fn(f64) -> f64 + Sync, n: usize) -> f64 {
    if a == b {
        return 0.;
    }

    let coefficients = get_coefficients(n);
    let step = (b - a) / (n - 1) as f64;

    (0..n)
        .into_par_iter()
        .map(|i| a + step * i as f64)
        .zip(coefficients)
        .map(|(x_i, c)| c * f(x_i))
        .sum::<f64>()
        * step
}

pub fn closed_newton_cotes(
    a: f64,
    b: f64,
    f: impl Fn(f64) -> f64 + Sync,
    n: usize,
    partition_count: usize,
) -> f64 {
    if a == b {
        return 0.;
    }

    let step = (b - a) / partition_count as f64;
    (0..partition_count + 1)
        .map(|i| a + step * i as f64)
        .tuple_windows::<(_, _)>()
        .par_bridge()
        .map(|(a_i, b_i)| _closed_newton_cotes(a_i, b_i, &f, n))
        .sum()
}
