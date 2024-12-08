use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn gen_rand_string(n: usize) -> String {
    let mut rng = StdRng::from_entropy();
    (0..n).map(|_| rng.gen_range('a'..='z')).collect()
}
