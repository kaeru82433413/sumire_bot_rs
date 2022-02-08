use rand::prelude::*;

pub fn round(value: f64) -> i32 {
    let div = value.div_euclid(1.) as i32;
    let rem = value.rem_euclid(1.);
    let mut rng = thread_rng();
    div + (rng.gen_bool(rem)) as i32
}