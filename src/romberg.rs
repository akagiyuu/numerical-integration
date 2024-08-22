use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn two_pow(n: usize) -> usize {
    1 << n
}

fn h(a: f64, b: f64, n: usize) -> f64 {
    (b - a) / two_pow(n) as f64
}

pub fn romberg(a: f64, b: f64, f: impl Fn(f64) -> f64 + Send + Sync + Copy, step: usize) -> f64 {
    let mut r = vec![vec![0.; step]; step];
    r[0][0] = (b - a) * (f(b) + f(a)) * 0.5;

    for i in 1..step {
        let h_i = h(a, b, i);
        r[i][0] = 0.5 * r[i - 1][0]
            + h_i
                * (1..=two_pow(i - 1))
                    .into_par_iter()
                    .map(|k| a + (2 * k - 1) as f64 * h_i)
                    .map(f)
                    .sum::<f64>();
    }

    for j in 1..step {
        for i in j..step {
            r[i][j] = (two_pow(2 * j) as f64 * r[i][j - 1] - r[i - 1][j - 1])
                / (two_pow(2 * j) - 1) as f64
        }
    }
    r[step - 1][step - 1]
}
