use rand::{rngs::ThreadRng, Rng};

pub fn weighted_random_choice<T: Clone>(a: &Vec<(usize, T)>, rng: &mut ThreadRng) -> T {
    let mut weights = Vec::new();

    for i in 0..a.len() {
        if i == 0 {
            weights.push(a[i].0);
        } else {
            weights.push(a[i].0 + weights[i - 1]);
        }
    }

    let rand = rng.gen_range(0..weights[weights.len() - 1]);
    let mut i = 0;
    for _ in 0..weights.len() {
        if weights[i] > rand {
            break;
        }
        i += 1;
    }
    return a[i].1.clone();
}

pub fn random_choice<T: Clone>(a: &Vec<(usize, T)>, rng: &mut ThreadRng) -> T {
    let rand = rng.gen_range(0..a.len());
    return a[rand].1.clone();
}