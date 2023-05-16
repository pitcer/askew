pub fn binomial_coefficient(n: u32, k: u32) -> u32 {
    ((n - k + 1)..=n).product::<u32>() / (1..=k).product::<u32>()
}

pub fn bernstein(n: u32, k: u32, t: f32) -> f32 {
    binomial_coefficient(n, k) as f32 * t.powi(k as i32) * (1.0 - t).powi((n - k) as i32)
}