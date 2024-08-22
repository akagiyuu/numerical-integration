use itertools::Itertools;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, ParallelBridge, ParallelIterator,
};

fn get_coefficients(n: usize) -> Vec<f64> {
    match n {
        1 => vec![2.],
        2 => vec![1.5, 1.5],
        3 => vec![8. / 3., -4. / 3., 8. / 3.],
        4 => vec![55. / 24., 5. / 24., 5. / 24., 55. / 24.],
        5 => vec![3.3, -4.2, 7.8, -4.2, 3.3],
        _ => unimplemented!(),
    }
}

fn _open_newton_cotes(a: f64, b: f64, f: impl Fn(f64) -> f64 + Sync, n: usize) -> f64 {
    if a == b {
        return 0.;
    }

    let coefficients = get_coefficients(n);
    let step = (b - a) / (n + 1) as f64;

    (1..n + 1)
        .into_par_iter()
        .map(|i| a + step * i as f64)
        .zip(coefficients)
        .map(|(x_i, c)| c * f(x_i))
        .sum::<f64>()
        * step
}

pub fn open_newton_cotes(
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
        .map(|(a_i, b_i)| _open_newton_cotes(a_i, b_i, &f, n))
        .sum()
}
